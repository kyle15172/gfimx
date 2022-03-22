use redis::{Commands, RedisError};

const HOST: Option<&str> = option_env!("REDIS_HOST");
const PORT: Option<&str> = option_env!("REDIS_PORT");
const NAME: &str = env!("CLIENT_NAME", "Please add a name for the FIM client in config.toml");

trait BrokerImpl {
    fn get_policy(&self) -> String;
}

pub enum BrokerType {
    Redis,    
}

pub struct BrokerProxy {
    _impl: Box<dyn BrokerImpl + Send>,
}

impl BrokerProxy {
    pub fn new(broker_type: BrokerType) -> Self {
        let b_type: Box<dyn BrokerImpl + Send> = match broker_type {
            BrokerType::Redis => Box::new(RedisBroker::new())
        };
        BrokerProxy { _impl: b_type }
    }

    pub fn get_policy(&self) -> String {
        self._impl.get_policy()
    }
}

struct RedisBroker {
    _client: redis::Client
}

impl RedisBroker {
    pub fn new() -> Self {
        let host = HOST.expect("REDIS_HOST variable not set!");
        let port = PORT.unwrap_or("6379");

        let _client = redis::Client::open(format!("redis://{}:{}/", host, port));
        if let Err(reason) = _client {
            panic!("Could not connect to Redis! Reason: {}", reason);
        }
        RedisBroker { _client: _client.unwrap() }
    }
}

impl BrokerImpl for RedisBroker {
    fn get_policy(&self) -> String {
        let mut conn = match self._client.get_connection() {
            Ok(conn) => conn,
            Err(reason) => panic!("Failed to connect to Redis: Reason {}", reason)
        };

        let val: Result<String, RedisError> = conn.get(format!("{}_policy", NAME));

        if let Err(reason) = val {
            panic!("Could not get policy for {}! Reason: {}", NAME, reason);
        }

        val.unwrap()
    }
}