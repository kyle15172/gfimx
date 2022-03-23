use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::mpsc::channel;

use crate::fs_scanner::FilesystemScanner;
use crate::policy_structs::Ignore;
pub struct DirWatcher{
    watch_dirs: Vec<String>,
    ignore_files: Option<Ignore>,
    ignore_dirs: Option<Ignore>,
    scanner: Arc<Mutex<FilesystemScanner>>
}

impl DirWatcher {

    pub fn new(watch_dirs: Vec<String>, 
               scanner: Arc<Mutex<FilesystemScanner>>, 
               ignore_files: Option<Ignore>, 
               ignore_dirs: Option<Ignore>) -> Self 
    {
        DirWatcher { watch_dirs, ignore_files, ignore_dirs, scanner }
    }

    pub fn watch(&self) {
        let (tx, rx) = channel();
        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher : RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.

        for dir in &self.watch_dirs {
            let res = watcher.watch(dir, RecursiveMode::Recursive);

            if let Err(r) = res {
                panic!("DirWatcher cannot watch directory {}! Reason: {}", dir, r)
            }
        }


        // This is a simple loop, but you may want to use more complex logic here,
        // for example to handle I/O.
        loop {
            match rx.recv() {
                Ok(event) => {

                    match event {
                        DebouncedEvent::Write(e) | DebouncedEvent::NoticeWrite(e)=> {

                            self.scanner.lock().unwrap().scan_dir(e.to_str().unwrap())
                        },
                        _ => {}
                        
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }    
}