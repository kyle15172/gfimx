use std::sync::mpsc::{Sender, RecvError};

use reflux::RefluxComputeNode;

use crate::structs::FileHash;

pub struct FileComparer {
    queue: RefluxComputeNode<FileHash,  ()>,
}

impl FileComparer {
    pub fn new() -> Self {
        let queue: RefluxComputeNode<FileHash,  ()> = RefluxComputeNode::new();
        FileComparer { queue }
    }

    pub fn collector(&self) -> Sender<FileHash> {
        self.queue.collector()
    }
    
    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> () {
        self.queue.set_computer(1, move|hash, _, _, _| {            
            println!("Name: {}\nHash:{}", hash.name, hash.hash);
            Ok(())
        }, ());
        self.queue.run(timeout)
    }
}

