use std::sync::{Arc, Mutex};
use std::{thread, time};

use crate::schedule::ISchedule;
use crate::fs_scanner::FilesystemScanner;

pub type SchedList = Vec<Box<dyn ISchedule + Send>>;
pub type FsScanner = Arc<Mutex<FilesystemScanner>>;

/// Contains a list of Schedules and a FilesystemScanner.
/// 
/// Periodically iterates through the list of schedules and
/// interrogates them. If a value is returned, the scanner is
/// invoked.
pub struct ScheduleRunner {
    schedules: SchedList,
    scanner: FsScanner
}

impl ScheduleRunner {

    /// Creates a new `ScheduleRunner` instance
    /// 
    /// # Arguments
    /// * `schedules` - A list of Schedules to interrogate
    /// * `scanner` - File scanner to invoke when running a schedule
    pub fn new (schedules: SchedList, scanner: FsScanner) -> Self {
        ScheduleRunner { schedules, scanner }
    }

    /// Iterates through a list of schedules and invokes the scanner if
    /// the interrogation returns a value.
    /// 
    /// This function runs indefinitely
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