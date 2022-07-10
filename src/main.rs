use std::{thread, sync::{Arc, Mutex}, time::Duration};

use dir_traverser::DirTraverser;
use file_comparer::FileComparer;
use file_hasher::FileHasher;
use file_reader::FileReader;
use mbfs::MessageBasedFileSystem;

mod dir_traverser;
mod file_comparer;
mod file_hasher;
mod file_reader;
mod structs;

fn main() -> thread::Result<()> {
    let mut dt = DirTraverser::new();
    let mut fr = FileReader::new();
    let mut fh = FileHasher::new();
    let mut fc = FileComparer::new();

    let file_reader = Arc::new(Mutex::new(MessageBasedFileSystem::new()));
    let run = file_reader.lock().unwrap().get_run_signal();
    let fr2 = file_reader.clone();

    let read_thr = thread::spawn(move || {
        loop {
            {
                fr2.lock().unwrap().run().unwrap();
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    dt.set_drain(fr.collector());
    fr.set_drain(fh.collector());
    fh.set_drain(fc.collector());

    let sender = dt.collector();
    let _ = sender.send("/home/kyleb/Documents".to_owned());


    let thr = thread::spawn(move || {
        fr.run(|signal| {
            let _ =  signal.send((60000, false));
        }).expect("Eish");
    });

    let thr2 = thread::spawn(move || {
        dt.run(|signal| {
            let _ =  signal.send((60000, false));
        }, file_reader).expect("Eish");
    });

    let thr3 = thread::spawn(move || {
        fh.run(|signal| {
            let _ =  signal.send((60000, false));
        }).expect("Eish");
    });

    let thr4 = thread::spawn(move || {
        fc.run(|signal| {
            let _ =  signal.send((60000, false));
        }).expect("Eish");
    });

    thr.join()?;
    thr2.join()?;
    thr3.join()?;
    thr4.join()?;
    run.store(false, std::sync::atomic::Ordering::Relaxed);
    read_thr.join()?;
    println!("e");
    Ok(())
}
