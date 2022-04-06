use std::{fmt::Display, cell::RefCell, sync::Mutex};

use flexi_logger::{writers::LogWriter, DeferredNow, Record};
use redis::Commands;

const HOST: Option<&str> = option_env!("REDIS_HOST");
const PORT: Option<&str> = option_env!("REDIS_PORT");
const NAME: &str = env!("CLIENT_NAME", "Please add a name for the FIM client in config.toml");

pub enum LoggerPlatforms {
    Redis,
    None
}

impl Display for LoggerPlatforms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LoggerPlatforms::Redis => write!(f, "Redis"),
            LoggerPlatforms::None => write!(f, "None")
        }
    }
}

trait LoggerBridgeImpl {
    fn log(&self, msg: String);    
    fn flush(&self);
}


pub struct LoggerBridge {
    _impl: Box<dyn LoggerBridgeImpl + Send + Sync>
}


impl LoggerBridge {
    pub fn new(platform: LoggerPlatforms) -> Result<Self, String> {
        let log_impl = match platform {
            LoggerPlatforms::Redis => {
                Box::new(RedisLogger::new())
            }
            _ => {
                return Err(format!("Platform {} not supported!", platform))
            }
        };

        Ok(LoggerBridge{ _impl: log_impl})
    }

    pub fn log(&self, msg: String) {
        self._impl.log(msg)
    }

    pub fn flush(&self) {
        self._impl.flush();
    }
}


struct RedisLogger {
    client: redis::Client,
    buffer: Mutex<RefCell<Vec<String>>>
}

impl RedisLogger {
    pub fn new() -> Self {
        let host = HOST.expect("REDIS_HOST variable not set!");
        let port = PORT.unwrap_or("6379");

        let _client = redis::Client::open(format!("redis://{}:{}/", host, port));
        if let Err(reason) = _client {
            panic!("Error creating Redis client: {}", reason)
        }
        let client = _client.unwrap();

        //checks if we can connect to redis
        if let Err(reason) = client.get_connection() {
            panic!("Could not connect to Redis! Reason: {}", reason)
        }

        let mut conn = client.get_connection().unwrap();

        if let Err(reason) = redis::cmd("PING").query::<String>(&mut conn) {
            panic!("Could not connect to Redis! Reason: {}", reason)
        }

        RedisLogger { 
            client, 
            buffer: Mutex::new(RefCell::new(Vec::new()))
        }
    }
}

impl LoggerBridgeImpl for RedisLogger {
    fn log(&self, msg: String) {
        let val_lock = self.buffer.lock().unwrap();
        let mut buf = val_lock.borrow_mut();
        buf.push(msg);
        
    } 

    fn flush(&self) {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(reason) => {panic!("Failed to connect to Redis: Reason {}", reason)}
        };

        let val_lock = self.buffer.lock().unwrap();
        let mut buf = val_lock.borrow_mut();

        for item in buf.iter() {
            let _: () = conn.lpush(format!("{}_log", NAME), item).expect("Eish...");
        }

        buf.clear();
    }
}

/// Custom LogWriter implementaion for `flexi_logger`, allowing
/// logs to be written to a `BrokerProxy` object.
pub struct BrokerLogWriter{
    logger: LoggerBridge
}

impl BrokerLogWriter {
    pub fn new(logger: LoggerBridge) -> Self {
        BrokerLogWriter { logger }
    }
}

impl LogWriter for BrokerLogWriter {
    fn write(&self, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        self.logger.log(format!("{} | {} => [{}]: {}", now.now(), record.target(), record.level(), record.args()));
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        self.logger.flush();
        Ok(())
    }
}

