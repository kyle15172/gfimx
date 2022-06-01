use std::cell::Cell;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
use std::sync::mpsc::{self, Sender, Receiver, RecvError};

pub struct ReflexiveQueue<T> {
    subscribers: Vec<(JoinHandle<()>, Sender<T>)>,
    receiver: Receiver<T>,
    sender: Sender<T>,
    run: Arc<Mutex<Cell<bool>>>,
}

impl<T:'static + std::marker::Send> ReflexiveQueue<T> {
    pub fn new() -> Self {        
        let (tx, rx) = mpsc::channel();
        ReflexiveQueue { subscribers: Vec::new(), receiver: rx, sender: tx, run: Arc::new(Mutex::new(Cell::new(true))) }
    }

    pub fn sink<F>(&mut self, n_sinks: usize, sink_fn: F,) -> ()
    where F: Fn(Sender<T>, &Receiver<T>) -> () + 'static + Copy + std::marker::Send
    {
        for _ in 0..n_sinks {
            let (tx, rx) = mpsc::channel();
            let own_sender = self.sender.clone();
            let run_lock = self.run.clone();
            self.subscribers.push((thread::spawn(move || {
                while run_lock.lock().unwrap().get() {
                    sink_fn(own_sender.clone(), &rx);
                }
            }), tx))
        }
        
    }

    pub fn source(&self, item: T) {
        let _ = self.sender.send(item);
    }

    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> ()
    {
        loop {
            for i in 0..self.subscribers.len() {
                let (tx, rx) = mpsc::channel();
                timeout(tx);
                let (tm, retry) = rx.recv()?;
                let val = self.receiver.recv_timeout(Duration::from_millis(tm));
                if val.is_ok() {
                    let _ = self.subscribers.get(i).unwrap().1.send(val.unwrap());
                } else if !retry{
                    self.run.lock().unwrap().set(false);
                    return Ok(());
                }
            }
        }
    }
}