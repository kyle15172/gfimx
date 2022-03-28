use std::collections::HashMap;

use serde_derive::Deserialize;

/// Top level dataclass, containing a `Watch` and a map of `Schedule` objects
#[derive(Deserialize, Debug)]
pub struct Policy {
    pub watch: Option<Watch>,
    pub schedule: Option<HashMap<String, Schedule>>,
}

/// Dataclass containing directories to be watched as 
/// well as file and directory filters.
#[derive(Deserialize, Debug)]
pub struct Watch {
    pub dirs: Vec<String>,
    pub ignore_files: Option<Ignore>,
    pub ignore_dirs: Option<Ignore>,
}

/// Dataclass containing paths and patterns to ignore.
#[derive(Deserialize, Debug , Clone)]
pub struct Ignore {
    pub patterns: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

/// Dataclass containing directories to monitor, file and directory filters
/// and a `cron` and `interval` schedule. 
/// 
/// Note: `cron` and `interval` are mutually exclusive.
#[derive(Deserialize, Debug)]
pub struct Schedule {
    pub dirs: Vec<String>,
    pub ignore_files: Option<Ignore>,
    pub ignore_dirs: Option<Ignore>,
    pub interval: Option<u32>,
    pub cron: Option<String>,
}

