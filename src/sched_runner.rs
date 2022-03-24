use std::sync::{Arc, Mutex};
use std::{thread, time};

use crate::schedule::ISchedule;
use crate::fs_scanner::FilesystemScanner;

type SchedList = Vec<Box<dyn ISchedule + Send>>;
type FsScanner = Arc<Mutex<FilesystemScanner>>;

pub struct ScheduleRunner {
    schedules: SchedList,
    scanner: FsScanner
}

impl ScheduleRunner {

    pub fn new (schedules: SchedList, scanner: FsScanner) -> Self {
        ScheduleRunner { schedules, scanner }
    }

    pub fn run(&mut self) {
        loop {
            thread::sleep(time::Duration::from_millis(100));
            for schedule in &mut self.schedules {


                if let Some(paths) = schedule.interrogate() {
                    for path in paths.0 {
                        let mut scanner = self.scanner.lock().unwrap();
                        scanner.scan_dir(path, paths.1, paths.2)
                    }
                }
            }
        }
    }
}