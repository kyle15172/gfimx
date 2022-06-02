use std::{sync::{mpsc::{Sender, RecvError}, Arc, Mutex}, io::{BufRead, Error, ErrorKind}, collections::HashMap};

use reflexive_queue::ReflexiveQueue;

use crate::structs::FileProgress;

pub struct FileReader {
    queue: ReflexiveQueue<FileProgress, Vec<u8>>,
    data_pile: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl FileReader {
    pub fn new(data_pile: Arc<Mutex<HashMap<String, Vec<u8>>>>) -> Self {
        let mut queue = ReflexiveQueue::new();
        queue.transform(4, move|mut prog: FileProgress, feedback, drainer| {            
            let reader = &mut prog.file;
            reader.seek_relative(prog.offset)?;
            let buffer = reader.fill_buf()?;
            let length = buffer.len();
            let _ = drainer.ok_or(Error::new(ErrorKind::BrokenPipe, "No drain set"))?.send(Vec::from(buffer));
            reader.consume(length);
            prog.offset += length as i64;
            prog.chunk_no += 1;
            
            if length > 0 {
                feedback.send(prog);
            }
            Ok(())
        });
        FileReader { queue, data_pile }
    }

    pub fn set_drain(&mut self, drain: Sender<Vec<u8>>) {
        self.queue.set_drain(drain);
    }
    
    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> () {
        self.queue.run(timeout)
    }
}