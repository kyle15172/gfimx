use std::thread;
use log::error;

mod broker_proxy;
mod schedule;
mod sched_runner;
mod fs_scanner;
mod watcher;
mod policy_structs;
mod monitor_builder;
mod file_metadata;
mod db_proxy;
mod logger;

use db_proxy::{DatabaseProxy, DatabaseType};
use flexi_logger::Logger;
use logger::BrokerLogWriter;
use policy_structs::Policy;
use monitor_builder::build_monitors;
use broker_proxy::{BrokerProxy, BrokerType};

fn main() {

    let broker = BrokerProxy::new(BrokerType::Redis);

    let _logger = Logger::try_with_str("info").unwrap()
        .log_to_writer(Box::new(BrokerLogWriter::new(broker.clone()))).start().expect("Eish...");

    let db = DatabaseProxy::new(DatabaseType::MongoDB);

    if let Err(reason) = &db {
        error!("Cannot connect to database: {}", reason);
    }

    let config = toml::from_str::<Policy>(broker.get_policy().as_str());

    if let Err(reason) = &config {
        error!("Cannot parse policy: {}", reason);
        panic!()
    }

    let (watcher, runner) = build_monitors(config.unwrap(), broker, db.unwrap());

    let thr1 = thread::spawn(move || {
        if let Some(_watch) = watcher {
            _watch.watch();
        }
    });

    let thr2 = thread::spawn(move || {
        if let Some(mut _run) = runner {
            _run.run();
        }
    });

    thr1.join().ok();
    thr2.join().ok();
}