use std::{sync::mpsc::{Sender, RecvError}, fs};

use reflexive_queue::ReflexiveQueue;

pub struct DirTraverser {
    queue: ReflexiveQueue<String, String>,
}

impl DirTraverser {
    pub fn new() -> Self {
        let mut queue = ReflexiveQueue::new();
        queue.transform(32, move|source, feedback, drainer| {
            loop {
                let val = source.recv();
                if val.is_ok() {
                    let dir = val.unwrap();
    
                    println!("{}", &dir);
    
                    let paths_result = fs::read_dir(dir);
    
                    if paths_result.is_err() {
                        continue;
                    }
    
                    let paths = paths_result.unwrap();
    
                    for path in paths {
    
                        let _path = path.unwrap();
    
                        if _path.file_type().unwrap().is_symlink() {
                            continue;
                        }
    
                        let entry = fs::metadata(_path.path()).unwrap();
                        if entry.is_dir() {
                            let _ = feedback.send(_path.path().display().to_string());
                        } else if entry.is_file() {
                            let _ = if let Some(ref drain) = drainer {
                                drain.send(_path.path().display().to_string())
                            } else {
                                Ok(())
                            };
                        }
                    }
                }
            }
        });
        DirTraverser { queue }
    }

    pub fn collector(&self) -> Sender<String> {
        self.queue.collector()
    }

    pub fn add_drain(&mut self, drain: Sender<String>) {
        self.queue.drain(drain);
    }
    
    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> () {
        self.queue.run(timeout)
    }
}