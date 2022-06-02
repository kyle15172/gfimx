use std::cell::Cell;
use std::sync::{Mutex, Arc};
use std::{thread, io};
use std::time::Duration;
use std::thread::JoinHandle;
use std::sync::mpsc::{self, Sender, Receiver, RecvError};

pub struct ReflexiveQueue<T, D> {
    transformers: Vec<(JoinHandle<()>, Sender<T>)>,
    receiver: Receiver<T>,
    sender: Sender<T>,
    run: Arc<Mutex<Cell<bool>>>,
    drainer: Option<Sender<D>>,
}

impl<T: 'static + std::marker::Send, D: 'static + std::marker::Send> ReflexiveQueue<T, D> {
    pub fn new() -> Self {        
        let (tx, rx) = mpsc::channel();
        ReflexiveQueue{
            transformers: Vec::new(),
            receiver: rx,
            sender: tx,
            run: Arc::new(Mutex::new(Cell::new(true))),
            drainer: None,
        }
    }

    pub fn transform<F>(&mut self, n_sinks: usize, sink_fn: F,) -> ()
    where F: Fn(T, Sender<T>, Option<Sender<D>>) -> io::Result<()> + 'static + Copy + std::marker::Send
    {
        for _ in 0..n_sinks {
            let (tx, rx) = mpsc::channel();
            let own_sender = self.sender.clone();
            let drainer = self.drainer.clone();
            let run_lock = self.run.clone();
            self.transformers.push((thread::spawn(move || {
                while run_lock.lock().unwrap().get() {
                    loop {
                        let val = rx.recv();
                        if val.is_err() {
                            break;
                        }
                        if let Err(_) = sink_fn(val.unwrap(), own_sender.clone(), drainer.clone()) {
                            break;
                        };
                    }
                }
            }), tx))
        }
        
    }

    pub fn collector(&self) -> Sender<T> {
        self.sender.clone()
    }

    pub fn set_drain(&mut self, drainer: Sender<D>) {
        self.drainer = Some(drainer);
    }

    pub fn run<F>(&mut self, timeout: F) -> Result<(), RecvError>
    where F: Fn(Sender<(u64, bool)>) -> ()
    {
        loop {
            for i in 0..self.transformers.len() {
                let (tx, rx) = mpsc::channel();
                timeout(tx);
                let (tm, retry) = rx.recv()?;
                let val = self.receiver.recv_timeout(Duration::from_millis(tm));
                if val.is_ok() {
                    let _ = self.transformers.get(i).unwrap().1.send(val.unwrap());
                } else if !retry{
                    self.run.lock().unwrap().set(false);
                    return Ok(());
                }
            }
        }
    }
}