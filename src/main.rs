use std::{sync::{Arc, Mutex}, vec, thread};


mod schedule;
mod sched_runner;
mod fs_scanner;
mod watcher;

fn main() {

    let paths1: Vec<String> = vec!["/home/kyleb/Public".to_owned(), "/home/kyleb/Documents/SCRATCHPAD".to_owned()];
    let paths2: Vec<String> = vec!["/home/kyleb/Documents/odoo".to_owned()];
    let paths3: Vec<String> = vec!["/home/kyleb/Documents/automation".to_owned()];

    let sched1: Box<dyn schedule::ISchedule + Send> = Box::new(schedule::CronSchedule::new("*/5 * * * *".to_string(), paths1, Vec::new()).unwrap());
    let sched2: Box<dyn schedule::ISchedule + Send> = Box::new(schedule::IntervalSchedule::new("120".to_owned(), paths2, Vec::new()).unwrap());

    let checker = Arc::new(Mutex::new(fs_scanner::FilesystemScanner{}));

    let mut runner = sched_runner::ScheduleRunner::new(vec![sched1, sched2], Arc::clone(&checker));
    let watcher = watcher::DirWatcher::new(paths3, Arc::clone(&checker));

    let thr1 = thread::spawn(move || {
        watcher.watch();
    });

    let thr2 = thread::spawn(move || {
        runner.run();
    });

    thr1.join().ok();
    thr2.join().ok();
}