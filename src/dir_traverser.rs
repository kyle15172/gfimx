use std::{sync::mpsc::{Sender, RecvError}, fs, io::{Error, self}};

use reflux::RefluxComputeNode;

use crate::structs::FileProgress;

pub struct DirTraverser {
    queue: RefluxComputeNode<String, FileProgress>,
}

impl DirTraverser {
    pub fn new() -> Self {
        let queue = RefluxComputeNode::new();
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
        self.queue.set_computer(16, move|dir, feedback, drainer, _| {    
            println!("{}", &dir);

            let clone = |item: &io::Result<Sender<FileProgress>>| -> io::Result<Sender<FileProgress>> {
                if item.is_ok() {
                    Ok(item.as_ref().unwrap().clone())
                } else {
                    let e_kind = item.as_ref().unwrap_err().kind();
                    let e_msg = item.as_ref().unwrap_err().to_string();
                    Err(Error::new(e_kind, e_msg))
                }
            };

            let paths = fs::read_dir(dir)?;

            let _drainer = &drainer;

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
                    let _ = clone(_drainer)?.send(file);
                }
            }
            Ok(())
        }, ());
        self.queue.run(timeout)
    }
}