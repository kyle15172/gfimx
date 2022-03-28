use std::{sync::{Arc, Mutex}, borrow::Borrow};
use log::error;

use crate::{
    policy_structs::Policy, 
    sched_runner::ScheduleRunner, 
    watcher::DirWatcher, 
    fs_scanner::FilesystemScanner, 
    schedule::{ISchedule, CronSchedule, IntervalSchedule},
    broker_proxy::BrokerProxy, db_proxy::DatabaseProxy
};

/// Builds a `DirWatcher` and `ScheduleRunner` object.
/// 
/// # Arguments
/// * `policy` - `Policy` dataclass to build the monitors
/// * `broker` - `BrokerProxy` object to use for message passing
/// * `database` - `DatabaseProxy` object to store and retrieve file information
/// 
/// # Returns
/// A tuple containing `DirWatcher` and `ScheduleRunner` optional objects.
pub fn build_monitors(policy: Policy, broker: BrokerProxy, database: DatabaseProxy) -> (Option<DirWatcher>, Option<ScheduleRunner>) {

    let checker = Arc::new(Mutex::new(FilesystemScanner::new(broker, database)));
    let watcher: Option<DirWatcher> = if let Some(watch_policy) = policy.watch {

        Some(DirWatcher::new(
            watch_policy.dirs, 
            Arc::clone(&checker), 
            watch_policy.ignore_files, 
            watch_policy.ignore_dirs
        ))

    } else {
        None
    };

    let sched_runner: Option<ScheduleRunner> = if let Some(schedules) = policy.schedule {

        let mut scheds: Vec<Box<dyn ISchedule + Send>> = Vec::new();

        for sched in schedules.values() {
            if sched.cron.is_some() && sched.interval.is_some() {
                error!("Cannot have a schedule with both a cron and interval schedule!");
                panic!();
            }

            //TODO: Ensure that a new CronSchedule is successfully created before pushing it. 
            if let Some(cron_sched) = sched.cron.borrow() {

                let new_sched = CronSchedule::new(
                    cron_sched.clone(), 
                    sched.dirs.clone(), 
                    sched.ignore_files.clone(), 
                    sched.ignore_dirs.clone()
                );

                if let Err(reason) = new_sched {
                    error!("Cannot make new CronSchedule: {}", reason);
                    panic!();
                }

                scheds.push(Box::new(new_sched.unwrap()));
            }

            if let Some(int_sched) = sched.interval {
                let new_sched = IntervalSchedule::new(
                    format!("{}", int_sched), 
                    sched.dirs.clone(), 
                    sched.ignore_files.clone(), 
                    sched.ignore_dirs.clone()
                );

                if let Err(reason) = new_sched {
                    error!("Cannot make new IntervalSchedule: {}", reason);
                    panic!();
                }

                scheds.push(Box::new(new_sched.unwrap()));
            }
        }

        Some(ScheduleRunner::new(scheds, Arc::clone(&checker)))

    } else {
        None
    };

    (watcher, sched_runner)
}