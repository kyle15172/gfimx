use notify::{RecommendedWatcher, Result, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::mpsc::channel;
use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};

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

                            // let input = File::open(&e)?;
                            // let reader = BufReader::new(input);
                            // let digest = sha256_digest(reader)?;

                            // println!("This boio was changed: {}\nIt's hash is: {}", e.to_str().unwrap(), HEXLOWER.encode(digest.as_ref()))
                        },
                        _ => {}
                        
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }    
}
    


fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}