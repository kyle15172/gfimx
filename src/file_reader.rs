use crossbeam_channel::{Sender, RecvError};
use mbfs::FileHandle;
use reflux::RefluxComputeNode;

use crate::structs::FileChunk;


pub struct FileReader {
    queue: RefluxComputeNode<FileHandle, FileChunk>,
}

impl FileReader {
    pub fn new() -> Self {
        let queue: RefluxComputeNode<FileHandle, FileChunk> = RefluxComputeNode::new();
        FileReader { queue }
    }

    pub fn set_drain(&mut self, drain: Sender<FileChunk>) {
        self.queue.set_drain(drain);
    }

    pub fn collector(&self) -> Sender<FileHandle> {
        self.queue.collector()
    }
    
    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> () {
        self.queue.set_computers(1, move|mut handle, feedback, drainer, _| {            
            let buffer = handle.read()?;
            let length = buffer.len();
            let to_send = if length == 0 {
                FileChunk{
                    file_name: handle.get_name(),
                    chunk: None
                }
            } else {
                let name = handle.get_name();
                let _ = feedback.send(handle);
                FileChunk {
                    file_name: name,
                    chunk: Some(buffer),
                }
            };
            let _ = drainer?.send(to_send);
            Ok(())
        }, ());
        self.queue.run(timeout)
    }
}
