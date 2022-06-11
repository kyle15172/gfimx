use std::{sync::{mpsc::{Sender, RecvError}, RwLock, Arc, Mutex}, collections::HashMap};

use reflux::RefluxComputeNode;
use sha2::{Sha256, Digest};
use data_encoding::HEXLOWER;

use crate::structs::{FileChunk, FileHash};

pub struct FileHasher {
    queue: RefluxComputeNode<FileChunk,  FileHash>,
}

impl FileHasher {
    pub fn new() -> Self {
        let queue: RefluxComputeNode<FileChunk, FileHash> = RefluxComputeNode::new();
        FileHasher { queue }
    }

    pub fn set_drain(&mut self, drain: Sender<FileHash>) {
        self.queue.set_drain(drain);
    }

    pub fn collector(&self) -> Sender<FileChunk> {
        self.queue.collector()
    }
    
    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> () {
        self.queue.set_computer(1, move|chunk, _, drainer, data_pile| {            
            let is_present = {
                data_pile.read().unwrap().get(&chunk.file_name).is_some()
            };
            if is_present {
                
                let x = data_pile.read().unwrap();
                let digest_guard = x.get(&chunk.file_name).unwrap();
                
                let mut digest = digest_guard.lock().unwrap();
                match chunk.chunk {
                    Some(data) => digest.update(data),
                    None => {
                        let x = digest.clone().finalize();
                        let _ = drainer?.send(FileHash { name: chunk.file_name, hash: HEXLOWER.encode(x.as_ref())});
                    }
                }
            } else {
                let mut x = data_pile.write().unwrap();
                let mut hasher = Sha256::new();
                hasher.update(chunk.chunk.unwrap());
                x.insert(chunk.file_name, Mutex::new(hasher));
            }
            Ok(())
        }, Digests::new(RwLock::new(HashMap::new())));
        self.queue.run(timeout)
    }
}

type Digests = Arc<RwLock<HashMap<String, Mutex<Sha256>>>>;