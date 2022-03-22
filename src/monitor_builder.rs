use std::{sync::{Arc, Mutex}, borrow::Borrow};

use crate::{policy_structs::Policy, sched_runner::ScheduleRunner, watcher::DirWatcher, fs_scanner::FilesystemScanner, schedule::{ISchedule, CronSchedule}};

pub fn build_monitors(policy: Policy) -> (Option<DirWatcher>, Option<ScheduleRunner>) {

    let checker = Arc::new(Mutex::new(FilesystemScanner{}));
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
                panic!("Can't have cron and interval!");
            }

            if let Some(cron_sched) = sched.cron.borrow() {
                scheds.push(Box::new(CronSchedule::new(cron_sched.clone(), 
                    sched.dirs.clone(), 
                    sched.ignore_files.clone(), 
                    sched.ignore_dirs.clone()).unwrap()))
            }
        }

        Some(ScheduleRunner::new(Vec::new(), Arc::clone(&checker)))

    } else {
        None
    };

    (watcher, sched_runner)
}