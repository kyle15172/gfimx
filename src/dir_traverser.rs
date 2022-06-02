use std::{sync::mpsc::{Sender, RecvError}, fs::{self, File}, io::{BufReader, Error, ErrorKind}};

use reflexive_queue::ReflexiveQueue;

use crate::structs::FileProgress;

pub struct DirTraverser {
    queue: ReflexiveQueue<String, FileProgress>,
}

impl DirTraverser {
    pub fn new() -> Self {
        let mut queue = ReflexiveQueue::new();
        queue.transform(16, move|dir, feedback, drainer| {    
            println!("{}", &dir);

            let paths = fs::read_dir(dir)?;

            for path in paths {

                let _path = path?;

                if _path.file_type()?.is_symlink() {
                    continue;
                }

                let entry = fs::metadata(_path.path()).unwrap();
                if entry.is_dir() {
                    let _ = feedback.send(_path.path().display().to_string());
                } else if entry.is_file() {
                    let file = FileProgress::new(_path.path().display().to_string())?;
                    let _ = drainer.as_ref().ok_or(Error::new(ErrorKind::BrokenPipe, "No drain set"))?.send(file);
                }
            }
            Ok(())
        });
        DirTraverser { queue }
    }

    pub fn collector(&self) -> Sender<String> {
        self.queue.collector()
    }

    pub fn set_drain(&mut self, drain: Sender<FileProgress>) {
        self.queue.set_drain(drain);
    }
    
    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> () {
        self.queue.run(timeout)
    }
}