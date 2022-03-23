use std::os::unix::fs::MetadataExt;
use notify::Result;
use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
use walkdir::WalkDir;

use crate::broker_proxy::BrokerProxy;
use crate::db_proxy::DatabaseProxy;

pub struct FilesystemScanner{
    broker: BrokerProxy,
    database: DatabaseProxy
}


impl FilesystemScanner {

    pub fn new(broker: BrokerProxy, database: DatabaseProxy) -> Self {
        FilesystemScanner { broker, database }
    }

    pub fn scan_dir(&self, dir: &str) {
        for entry in WalkDir::new(dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok()) 
        {
            if !entry.metadata().unwrap().is_file() {
                continue;                
            }
            let path = entry.path().to_string_lossy();
            let attrs = entry.metadata().unwrap().mode();
            let uid = entry.metadata().unwrap().uid();
            let gid = entry.metadata().unwrap().gid();

            let input = File::open(entry.path()).unwrap();
            let reader = BufReader::new(input);
            let digest = sha256_digest(reader).unwrap();
            let hash = HEXLOWER.encode(digest.as_ref());

            let res:Option<()> = None;

            //TODO: Create FileMetadata object from existing data and compare the one you get from redis
            if res.is_none() {
                println!("{} : {:o} {} {} {}", path, attrs, uid, gid, hash);
            }
        }
    }
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}