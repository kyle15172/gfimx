use std::sync::{Arc, Mutex};
use log::error;

use redis::{Commands, RedisError};

const HOST: Option<&str> = option_env!("REDIS_HOST");
const PORT: Option<&str> = option_env!("REDIS_PORT");
const NAME: &str = env!("CLIENT_NAME", "Please add a name for the FIM client in config.toml");

/// Trait that all Broker implementations will use
trait BrokerImpl {

    /// Retrieve the policy from the Broker
    fn get_policy(&self) -> String;

    /// Log a message to the Broker
    fn log(&self, msg: String);
}

/// Enum for choosing which Broker to use
pub enum BrokerType {
    Redis,    
}

/// Proxy object for the rest of the system to interface with brokers.
/// 
/// Holds an implementation class that it uses to perform the desired
/// operation
pub struct BrokerProxy {
    _impl: Arc<Mutex<dyn BrokerImpl + Send>>,
}

impl BrokerProxy {
    pub fn new(broker_type: BrokerType) -> Self {
        let b_type: Arc<Mutex<dyn BrokerImpl + Send>> = match broker_type {
            BrokerType::Redis => Arc::new(Mutex::new(RedisBroker::new()))
        };
        BrokerProxy { _impl: b_type }
    }

    /// Retrieve the policy from the Broker
    pub fn get_policy(&self) -> String {
        self._impl.lock().expect("Eish...").get_policy()
    }

    /// Log a message to the Broker
    pub fn log(&self, msg: String) {
        self._impl.lock().expect("Eish...").log(msg);
    }
}

impl Clone for BrokerProxy {
    fn clone(&self) -> Self {
        Self { _impl: Arc::clone(&self._impl) }
    }
}

/// Redis implementation for `BrokerImpl`
struct RedisBroker {
    _client: redis::Client
}

impl RedisBroker {

    /// Connects to Redis and returns an instance of this object
    pub fn new() -> Self {
        let host = HOST.expect("REDIS_HOST variable not set!");
        let port = PORT.unwrap_or("6379");

        let _client = redis::Client::open(format!("redis://{}:{}/", host, port));
        if let Err(reason) = _client {
            error!("Could not connect to Redis! Reason: {}", reason);
            panic!()
        }
        RedisBroker { _client: _client.unwrap() }
    }

    /// Helper method to get a value from Redis
    /// 
    /// # Arguments
    /// * `query` - The key to query
    /// 
    /// # Returns
    /// * `Ok(String)` - The value returned from Redis if the query was successful
    /// * `Err(RedisError)` - Error that Redis has thrown if an error occured
    fn _get(&self, query: &str) -> Result<String, RedisError> {
        let mut conn = match self._client.get_connection() {
            Ok(conn) => conn,
            Err(reason) => {error!("Failed to connect to Redis: Reason {}", reason); panic!()}
        };

        conn.get(query)
    }
}

impl BrokerImpl for RedisBroker {
    fn get_policy(&self) -> String {
        
        let val = self._get(format!("{}_policy", NAME).as_str());

        if let Err(reason) = val {
            error!("Could not get policy for {}! Reason: {}", NAME, reason);
            panic!()
        }

        val.unwrap()
    }

    fn log(&self, msg: String) {
        let mut conn = match self._client.get_connection() {
            Ok(conn) => conn,
            Err(reason) => {error!("Failed to connect to Redis: Reason {}", reason); panic!()}
        };

        let _: () = conn.lpush(format!("{}_log", NAME), msg).expect("Eish...");
    }    
}