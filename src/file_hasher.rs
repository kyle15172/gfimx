use std::{sync::{Arc, Mutex}, collections::HashMap};
use crossbeam_channel::{Sender, RecvError};
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
        self.queue.set_computers(16, move|chunk, _, drainer, data_pile| {    
            let mut the_pile = data_pile.lock().unwrap();

            let is_present = {
                the_pile.get(&chunk.file_name).is_some()
            };
            if is_present {
                let mut to_delete = "".to_owned();
                {
                    let digest = the_pile.get_mut(&chunk.file_name).unwrap();
                    match chunk.chunk {
                        Some(data) => digest.update(data),
                        None => {
                            let x = digest.clone().finalize();
                            to_delete = chunk.file_name.clone();
                            let _ = drainer?.send(FileHash { name: chunk.file_name, hash: HEXLOWER.encode(x.as_ref())});
                        }
                    }
                }
                if to_delete.len() > 0 {
                    the_pile.remove(&to_delete);
                }
            } else {
                let mut hasher = Sha256::new();
                match chunk.chunk {
                    Some(val) => hasher.update(val),
                    None => {}
                }
                the_pile.insert(chunk.file_name, hasher);
            }
            Ok(())
        }, Digests::new(Mutex::new(HashMap::new())));
        self.queue.run(timeout)
    }
}

type Digests = Arc<Mutex<HashMap<String, Sha256>>>;
