use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bin_path: String,
    pub data_path: String,
    pub repackers: HashMap<String, RepackerPathsData>,
}

pub enum AppMode {
    Dev,
    User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepackerPathsData {
    pub from: String,
    pub to: String,
}
