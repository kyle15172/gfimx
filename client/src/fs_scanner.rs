use std::os::unix::fs::MetadataExt;
use std::path::Path;
use notify::Result;
use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
use walkdir::WalkDir;
use log::{error, info};
use regex::Regex;

use crate::broker_proxy::BrokerProxy;
use crate::db_proxy::DatabaseProxy;
use crate::file_metadata::FileMetadata;
use crate::policy_structs::Ignore;

pub struct FilesystemScanner{
    _broker: BrokerProxy,
    database: DatabaseProxy
}


impl FilesystemScanner {

    pub fn new(broker: BrokerProxy, database: DatabaseProxy) -> Self {
        FilesystemScanner { _broker: broker, database }
    }

    pub fn scan_dir(&mut self, dir: &str, ignore_files: &Option<Ignore>, ignore_dirs: &Option<Ignore>) {
        for entry in WalkDir::new(dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok()) 
        {

            if !entry.metadata().unwrap().is_file() {
                continue;                
            }

            let pattern_filter = |ignore_struct: Option<Ignore>, e_path: &Path, e_pattern: &str| -> Option<()> {

                let struct2 = ignore_struct.clone();

                for path in ignore_struct?.paths? {
                    if e_path.starts_with(path) { return Some(()) };
                }

                for pattern in struct2?.patterns? {
                    let reg = Regex::new(pattern.as_str()).unwrap_or_else(
                        |e: regex::Error| -> Regex {
                            error!("Cannot compile pattern: {} Reason: {}", pattern, e); 
                            panic!()
                        }
                    );
                    if reg.is_match(e_pattern) {
                        return Some(());
                    }
                }
                None
            };

            if pattern_filter(ignore_dirs.clone(), entry.path(), entry.path().parent().unwrap().to_str().unwrap()).is_some() {
                info!("Ignored file: {}", entry.path().to_string_lossy());
                continue;
            }

            if pattern_filter(ignore_files.clone(), entry.path(), entry.file_name().to_str().unwrap()).is_some() {
                info!("Ignored file: {}", entry.path().to_string_lossy());
                continue;
            }

            let path = entry.path().to_string_lossy();
            let perms = entry.metadata().unwrap().mode();
            let uid = entry.metadata().unwrap().uid();
            let gid = entry.metadata().unwrap().gid();

            let open_result = File::open(entry.path());

            if open_result.is_err() {
                error!("Cannot work with file {}: Reason: {}", path, open_result.err().unwrap());
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
                error!("Error fetching remote file info: {}", reason);
                continue;
            }

            if remote.as_ref().unwrap().is_none() {
                println!("{}", local);

                if let Err(reason) = self.database.upsert(local) {
                    error!("Could not upsert data! Reason: {}", reason);
                }
            } else if remote.as_ref().unwrap().as_ref().unwrap() != &local {
                println!("Old: {}\nNew: {}", remote.unwrap().unwrap(), local);
                if let Err(reason) = self.database.upsert(local) {
                    error!("Could not upsert data! Reason: {}", reason)
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