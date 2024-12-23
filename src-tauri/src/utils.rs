use std::collections::HashMap;

use map_modifier::{Map, Quest};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bin_path: String,
    pub data_path: String,
    pub mod_path: String,
    pub repackers: HashMap<String, RepackerPathsData>,
    pub maps: Vec<Map>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuntimeConfig {
    pub current_selected_map: Option<u16>
}

#[derive(Debug)]
pub struct LocalAppManager {
    pub runtime_config: Mutex<RuntimeConfig>,
    pub db_pool: RwLock<sqlx::SqlitePool>
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QuestProgressDBModel {
    pub id: Uuid,
    pub quest_id: Uuid,
    pub number: u32,
    pub text: String,
    pub concatenate: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestProgressFrontendModel {
    pub text: String,
    pub concatenate: bool
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QuestDBModel {
    pub id: Uuid,
    pub directory: String,
    pub campaign_number: u32,
    pub mission_number: u32,
    pub name: String,
    pub desc: String,
    pub script_name: String,
    pub is_active: bool,
    pub is_secondary: bool,
    pub is_first_init: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestFrontendModel {
    pub id: Uuid,
    pub directory: String,
    pub name: String,
    pub desc: String,
    pub script_name: String
}

impl From<QuestDBModel> for QuestFrontendModel {
    fn from(value: QuestDBModel) -> Self {
        QuestFrontendModel { 
            id: value.id, 
            directory: value.directory, 
            name: value.name, 
            desc: value.desc, 
            script_name: value.script_name 
        }
    }
}