use flexi_logger::{writers::LogWriter, DeferredNow, Record};

use crate::broker_proxy::BrokerProxy;

/// Custom LogWriter implementaion for `flexi_logger`, allowing
/// logs to be written to a `BrokerProxy` object.
pub struct BrokerLogWriter{
    broker: BrokerProxy
}

impl BrokerLogWriter {
    pub fn new(broker: BrokerProxy) -> Self {
        BrokerLogWriter { broker }
    }
}

impl LogWriter for BrokerLogWriter {
    fn write(&self, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        self.broker.log(format!("{} | {} => [{}]: {}", now.now(), record.target(), record.level(), record.args()));
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }
}

