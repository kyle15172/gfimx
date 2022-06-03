use std::{sync::{mpsc::{Sender, RecvError}}, io::BufRead};

use reflexive_queue::ReflexiveQueue;

use crate::structs::{FileProgress, FileChunk};

pub struct FileReader {
    queue: ReflexiveQueue<FileProgress, FileChunk>,
}

impl FileReader {
    pub fn new() -> Self {
        let queue: ReflexiveQueue<FileProgress, FileChunk> = ReflexiveQueue::new();
        FileReader { queue }
    }

    pub fn set_drain(&mut self, drain: Sender<FileChunk>) {
        self.queue.set_drain(drain);
    }

    pub fn collector(&self) -> Sender<FileProgress> {
        self.queue.collector()
    }
    
    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> () {
        self.queue.transform(1, move|mut prog, feedback, drainer, _| {            
            let reader = &mut prog.file;
            let buffer = reader.fill_buf()?.to_vec();
            let length = buffer.len();
            reader.consume(length);
            prog.chunk_no += 1;
            let to_send = if prog.chunk_no >= prog.chunks && length == 0 {
                FileChunk{
                    file_name: prog.name.clone(),
                    chunk: None
                }
            } else {
                let name = prog.name.clone();
                let _ = feedback.send(prog);
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
