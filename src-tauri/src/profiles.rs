use crate::services::version_control::VersionControlSimpleType;
use crate::utils::RepackerPathsData;
use map_modifier::Map;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct SheetEntity {
    pub spreadsheet_id: String,
    pub sheet_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub exe_name: PathBuf,
    pub game_path: PathBuf,
    pub map_path: PathBuf,
    pub texts_path: PathBuf,
    pub repackers: HashMap<String, RepackerPathsData>,
    pub maps: Vec<Map>,
    pub global_sheets: HashMap<VersionControlSimpleType, SheetEntity>,
}

impl ProfileConfig {
    pub fn get_map_spreadsheet_id(&self, map_id: i32) -> String {
        self.maps.iter().find(|m| m.id == map_id).unwrap().fights_spreadsheet_id.clone()
    }
}
