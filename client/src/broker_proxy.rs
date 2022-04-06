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

    fn log_details(&self, name: String, value: String);
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

    pub fn log_details(&self, name: String, value: String) {
        self._impl.lock().expect("Eish...").log_details(name, value)
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
            error!("Error creating Redis client: {}", reason);
            panic!()
        }
        let client = _client.unwrap();

        //checks if we can connect to redis
        if let Err(reason) = client.get_connection() {
            error!("Could not connect to Redis! Reason: {}", reason);
            panic!()
        }

        let mut conn = client.get_connection().unwrap();

        if let Err(reason) = redis::cmd("PING").query::<String>(&mut conn) {
            error!("Could not connect to Redis! Reason: {}", reason);
            panic!()
        }

        RedisBroker { _client: client }
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

    fn _set(&self, key: &str, val: &str) -> Result<(), RedisError> {
        let mut conn = match self._client.get_connection() {
            Ok(conn) => conn,
            Err(reason) => {error!("Failed to connect to Redis: Reason {}", reason); panic!()}
        };

        conn.set(key, val)
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

    fn log_details(&self, name: String, value: String) {
        let result = self._set(name.as_str(), value.as_str());

        if let Err(reason) = result {
            error!("Could not set value! Reason: {}", reason);
            panic!();
        }
    }
}