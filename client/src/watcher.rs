use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::mpsc::channel;
use log::error;

use crate::fs_scanner::FilesystemScanner;
use crate::policy_structs::Ignore;

/// This structure places filesystem watchers on provided directories
/// and calls the file scanner if any changes to watched files are detected.
/// 
/// Watches are recursively placed on provided directories. If a file changes
/// in a subdirectory of a provided directory, the change will be noted.
pub struct DirWatcher{
    watch_dirs: Vec<String>,
    ignore_files: Option<Ignore>,
    ignore_dirs: Option<Ignore>,
    scanner: Arc<Mutex<FilesystemScanner>>
}

impl DirWatcher {

    /// Returns an instance of a `DirWatcher`
    /// 
    /// # Arguments
    /// * `watch_dirs` - A list of directories to places watches on
    /// * `scanner` - A scanner object to invoke when changes are detected
    /// * `ignore_files` - File paths and patterns to ignore
    /// * `ignore_dirs` - Directory paths and patterns to ignore
    pub fn new(watch_dirs: Vec<String>, 
               scanner: Arc<Mutex<FilesystemScanner>>, 
               ignore_files: Option<Ignore>, 
               ignore_dirs: Option<Ignore>) -> Self 
    {
        DirWatcher { watch_dirs, ignore_files, ignore_dirs, scanner }
    }

    /// Monitors directories for any file changes.
    /// 
    /// This function runs indefinitely.
    pub fn watch(&self) {
        let (tx, rx) = channel();
        let mut watcher : RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

        // Add watches to given directories
        for dir in &self.watch_dirs {
            let res = watcher.watch(dir, RecursiveMode::Recursive);

            if let Err(r) = res {
                error!("DirWatcher cannot watch directory {}! Reason: {}", dir, r);
                panic!()
            }
        }

        // Monitor directories for any changes
        loop {
            match rx.recv() {
                Ok(event) => {

                    if let DebouncedEvent::NoticeWrite(e) = event {

                        self.scanner.lock().unwrap().scan_dir(e.to_str().unwrap(), &self.ignore_files, &self.ignore_dirs)
                    }
                }
                Err(e) => error!("watch error: {:?}", e),
            }
        }
    }    
}