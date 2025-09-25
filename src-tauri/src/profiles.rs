use std::collections::HashMap;

use derive_more::derive::Display;
use map_modifier::Map;
use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::utils::RepackerPathsData;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, EnumString, Display, Clone, Copy)]
pub enum ProfileType {
    Main,
    Hrta
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub session_configs_path: Option<String>,
    pub exe_name: String,
    pub bin_path: String,
    pub data_path: String,
    pub mod_path: String,
    pub texts_path: String,
    pub repackers: HashMap<String, RepackerPathsData>,
    pub maps: Vec<Map>
}