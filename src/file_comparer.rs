use std::sync::mpsc::{Sender, RecvError};

use reflexive_queue::ReflexiveQueue;

use crate::structs::FileHash;

pub struct FileComparer {
    queue: ReflexiveQueue<FileHash,  ()>,
}

impl FileComparer {
    pub fn new() -> Self {
        let queue: ReflexiveQueue<FileHash,  ()> = ReflexiveQueue::new();
        FileComparer { queue }
    }

    pub fn collector(&self) -> Sender<FileHash> {
        self.queue.collector()
    }
    
    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> () {
        self.queue.transform(1, move|hash, _, _, _| {            
            println!("Name: {}\nHash:{}", hash.name, hash.hash);
            Ok(())
        }, ());
        self.queue.run(timeout)
    }
}

