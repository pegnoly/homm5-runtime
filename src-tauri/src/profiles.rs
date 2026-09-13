use std::collections::HashMap;
use std::path::PathBuf;
use map_modifier::Map;
use serde::{Deserialize, Serialize};

use crate::utils::RepackerPathsData;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub exe_name: PathBuf,
    pub game_path: PathBuf,
    pub map_path: PathBuf,
    pub mod_path: PathBuf,
    pub texts_path: PathBuf,
    pub repackers: HashMap<String, RepackerPathsData>,
    pub maps: Vec<Map>
}