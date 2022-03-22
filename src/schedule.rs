use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Utc;
use cron_parser::parse;

use crate::policy_structs::Ignore;

pub trait ISchedule {
    /// Interrogates a schedule. If the schedule is due to be run, a list of paths will be returned.
    fn interrogate(&mut self) -> Option<(Vec<&String>, &Option<Ignore>, &Option<Ignore>)>;
}

/// Holds the data that scheduling classes will use.
/// This includes the schedule (either an interval or a cron schedule)
/// The paths that must be checked and any filters that should be applied
/// to ignore certain file or folder patterns.
struct Schedule {
    schedule: String,
    paths: Vec<String>,
    ignore_files: Option<Ignore>,
    ignore_dirs: Option<Ignore>,
}

/// This schedule object uses a time interval to determine whether it should be run or not.
pub struct IntervalSchedule {
    schedule: Schedule,
    next_run: u64
}

impl IntervalSchedule {
    pub fn new(schedule: String, 
               paths: Vec<String>, 
               ignore_files: Option<Ignore>, 
               ignore_dirs: Option<Ignore>) -> Result<Self, String> 
    {

        let sched_interval = schedule.parse::<u64>();

        if sched_interval.is_err() {
            return Err(format!("IntervalSchedule expected integer interval, got {}", schedule))
        }

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH).unwrap().as_secs();


        Ok(IntervalSchedule {
            schedule: Schedule{
                schedule,
                paths,
                ignore_files,
                ignore_dirs
            },
            next_run: since_the_epoch + sched_interval.unwrap()
        })
    }
}

impl ISchedule for IntervalSchedule {
    fn interrogate(&mut self) -> Option<(Vec<&String>, &Option<Ignore>, &Option<Ignore>)> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        if since_the_epoch > self.next_run {
            self.next_run = self.schedule.schedule.parse::<u64>().unwrap() + since_the_epoch;
            Some((self.schedule.paths.iter().filter_map(|e| {
                Some(e)
            }).collect(), &self.schedule.ignore_files, &self.schedule.ignore_dirs))
        } else {
            None
        }
    }
}

pub struct CronSchedule {
    schedule: Schedule
}

impl CronSchedule {
    pub fn new(schedule: String, 
               paths: Vec<String>, 
               ignore_files: Option<Ignore>, 
               ignore_dirs: Option<Ignore>) -> Result<Self, String> 
    {

        //Ensure that the cron schedule passed is valid before constructing the object
        if let Err(_) = parse(&schedule, &Utc::now()) {
            return Err(format!("CronSchedule expected cron schedule, got {}", schedule))
        }

        Ok(CronSchedule{
            schedule: Schedule { 
                schedule, 
                paths, 
                ignore_files,
                ignore_dirs
            }
        })    
    }
}

impl ISchedule for CronSchedule {
    fn interrogate(&mut self) -> Option<(Vec<&String>, &Option<Ignore>, &Option<Ignore>)> {

        let cron_sched: u64 = parse(&self.schedule.schedule, &Utc::now()).unwrap().timestamp().try_into().unwrap();
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH).unwrap().as_secs();

        if since_the_epoch >= cron_sched {
            Some((self.schedule.paths.iter().filter_map(|e| {
                Some(e)
            }).collect::<Vec<&String>>(), &self.schedule.ignore_files, &self.schedule.ignore_dirs))
        } else {
            None
        }
    }
}
