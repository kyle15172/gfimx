use walkdir::WalkDir;
use crate::broker_proxy::BrokerProxy;

pub struct FilesystemScanner{
    broker: BrokerProxy
}

impl FilesystemScanner {

    pub fn new(broker: BrokerProxy) -> Self {
        FilesystemScanner { broker }
    }

    pub fn scan_dir(&self, dir: &str) {
        for entry in WalkDir::new(dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok()) 
        {
            let f_name = entry.file_name().to_string_lossy();
            println!("{}", f_name)
        }
    }
}