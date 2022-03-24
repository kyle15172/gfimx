use std::collections::HashMap;

use serde_derive::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Policy {
    pub watch: Option<Watch>,
    pub schedule: Option<HashMap<String, Schedule>>,
}

#[derive(Deserialize, Debug)]
pub struct Watch {
    pub dirs: Vec<String>,
    pub ignore_files: Option<Ignore>,
    pub ignore_dirs: Option<Ignore>,
}

#[derive(Deserialize, Debug , Clone)]
pub struct Ignore {
    pub patterns: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Schedule {
    pub dirs: Vec<String>,
    pub ignore_files: Option<Ignore>,
    pub ignore_dirs: Option<Ignore>,
    pub interval: Option<u32>,
    pub cron: Option<String>,
}

