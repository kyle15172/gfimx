use std::{sync::{Arc, Mutex}, fs, io::{Error, self}, thread, time::Duration};

use crossbeam_channel::{Sender, RecvError};
use mbfs::{MessageBasedFileSystem, FileHandle};
use reflux::RefluxComputeNode;


pub struct DirTraverser {
    queue: RefluxComputeNode<String, FileHandle>,
}

impl DirTraverser {
    pub fn new() -> Self {
        let queue = RefluxComputeNode::new();
        DirTraverser { queue }
    }

    pub fn collector(&self) -> Sender<String> {
        self.queue.collector()
    }

    pub fn set_drain(&mut self, drain: Sender<FileHandle>) {
        self.queue.set_drain(drain);
    }
    
    pub fn run<F>(&mut self, timeout: F, file_reader: Arc<Mutex<MessageBasedFileSystem>>) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> () {
        self.queue.set_computers(4, move|dir, feedback, drainer, fs| {    
            let clone = |item: &io::Result<Sender<FileHandle>>| -> io::Result<Sender<FileHandle>> {
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
                    let handle = fs.lock().unwrap().open(_path.path().display().to_string());
                    let cln = clone(_drainer)?;
                    let _ = cln.send(handle);
                    drop(cln);
                }
                thread::sleep(Duration::from_millis(50));
            }
            Ok(())
        }, file_reader);
        self.queue.run(timeout)
    }
}