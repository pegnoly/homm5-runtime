use std::collections::HashMap;

use map_modifier::{Map, Quest};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

use crate::services::QuestService;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bin_path: String,
    pub data_path: String,
    pub mod_path: String,
    pub texts_path: String,
    pub repackers: HashMap<String, RepackerPathsData>,
    pub maps: Vec<Map>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuntimeConfig {
    pub current_selected_map: Option<u16>
}

#[derive(Debug)]
pub struct LocalAppManager {
    pub runtime_config: Mutex<RuntimeConfig>
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapFrontendModel {
    pub id: u16,
    pub name: String
}