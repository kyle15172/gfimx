use dir_traverser::DirTraverser;

mod dir_traverser;
mod file_reader;

fn main() {
    let mut dt = DirTraverser::new();
    let sender = dt.collector();
    let _ = sender.send("/home/kyleb".to_owned());
    dt.run(|signal| {
        let _ =  signal.send((1500, false));
     }).expect("Eish");
}
