use std::{sync::mpsc::{Sender, RecvError}, fs::{self, File}, io::{BufReader, BufRead}};

use reflexive_queue::ReflexiveQueue;

pub struct FileReader {
    queue: ReflexiveQueue<String, Vec<u8>>,
}

impl FileReader {
    pub fn new() -> Self {
        let mut queue = ReflexiveQueue::new();
        queue.transform(32, move|source, _, drainer| {
            loop {
                let val = source.recv();
                if val.is_ok() {
                    let file = File::open(val.unwrap());
                    if file.is_err() {
                        continue;
                    }
                    let mut reader = BufReader::with_capacity(4096, file.unwrap());

                    loop {
                        let length = {
                            let buffer = reader.fill_buf();
                            if buffer.is_ok() && drainer.is_some() {
                                let buf = buffer.unwrap();
                                let len = buf.len();
                                let _ = drainer.as_ref().unwrap().send(Vec::from(buf));
                                len
                            } else {
                                0
                            }
                        };
                        if length == 0 {
                            break;
                        }
                        reader.consume(length);
                    }
                }
            }
        });
        FileReader { queue }
    }

    pub fn add_drain(&mut self, drain: Sender<Vec<u8>>) {
        self.queue.drain(drain);
    }
    
    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> () {
        self.queue.run(timeout)
    }
}