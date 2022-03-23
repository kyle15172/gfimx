use std::os::unix::fs::MetadataExt;
use notify::Result;
use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
use walkdir::WalkDir;

use crate::broker_proxy::BrokerProxy;
use crate::db_proxy::DatabaseProxy;
use crate::file_metadata::FileMetadata;

pub struct FilesystemScanner{
    broker: BrokerProxy,
    database: DatabaseProxy
}


impl FilesystemScanner {

    pub fn new(broker: BrokerProxy, database: DatabaseProxy) -> Self {
        FilesystemScanner { broker, database }
    }

    pub fn scan_dir(&mut self, dir: &str) {
        for entry in WalkDir::new(dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok()) 
        {
            if !entry.metadata().unwrap().is_file() {
                continue;                
            }
            let path = entry.path().to_string_lossy();
            let perms = entry.metadata().unwrap().mode();
            let uid = entry.metadata().unwrap().uid();
            let gid = entry.metadata().unwrap().gid();

            let open_result = File::open(entry.path());

            if open_result.is_err() {
                self.broker.log(format!("Cannot work with file {}: Reason: {}", path, open_result.err().unwrap()));
                continue;
            }

            let input = open_result.unwrap();
            let reader = BufReader::new(input);
            let digest = sha256_digest(reader).unwrap();
            let hash = HEXLOWER.encode(digest.as_ref());

            let local = FileMetadata {
                path: format!("{}", path),
                uid,
                gid,
                perms,
                hash                
            };

            let remote = self.database.get_file(format!("{}", path).as_str());

            if let Err(reason) = &remote {
                self.broker.log(format!("Error fetching remote file info: {}", reason));
                continue;
            }

            if remote.as_ref().unwrap().is_none() {
                println!("{}", local);

                if let Err(reason) = self.database.upsert(local) {
                    self.broker.log(format!("Could not upsert data! Reason: {}", reason));
                }
            } else if remote.as_ref().unwrap().as_ref().unwrap() != &local {
                println!("Old: {}\nNew: {}", remote.unwrap().unwrap(), local);
                if let Err(reason) = self.database.upsert(local) {
                    self.broker.log(format!("Could not upsert data! Reason: {}", reason))
                }
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