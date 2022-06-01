use std::fs;

use reflexive_queue::ReflexiveQueue;

fn main() {
    let mut ol = ReflexiveQueue::new();
    ol.source("/home/kyleb".to_owned());
    ol.source("/usr".to_owned());
    ol.sink(32, move|tx, rx| {
        loop {
            let val = rx.recv();
            if val.is_ok() {
                let dir = val.unwrap();

                println!("{}", &dir);

                let paths_result = fs::read_dir(dir);

                if paths_result.is_err() {
                    continue;
                }

                let paths = paths_result.unwrap();

                for path in paths {

                    let _path = path.unwrap();

                    if _path.file_type().unwrap().is_symlink() {
                        continue;
                    }

                    let entry = fs::metadata(_path.path()).unwrap();
                    if entry.is_dir() {
                        let _ = tx.send(_path.path().display().to_string());
                    }
                }
            }
        }
    });
    ol.run(|signal| {
       let _ =  signal.send((1500, false));
    }).expect("Eish");
}
