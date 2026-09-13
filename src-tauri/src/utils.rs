use crate::{error::Error, profiles::ProfileConfig};
use map_modifier::{
    MapData,
    artifacts::ArtifactConfigEntity,
    buildings::{BankConfigEntity, BuildingConfigEntity},
};
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    path::{Path, PathBuf},
};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub current_profile: String,
    #[serde(skip)]
    pub generic_hero_xdb: PathBuf,
    #[serde(skip)]
    pub generic_icon_128: PathBuf,
    #[serde(skip)]
    pub generic_icon_dds: PathBuf,
    #[serde(skip)]
    pub generic_map_xdb: PathBuf,
    #[serde(skip)]
    pub session_configs_path: PathBuf,
    #[serde(skip)]
    pub auth_path: PathBuf
}

impl GlobalConfig {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let cfg_string = std::fs::read_to_string(path.join("main.json"))?;
        let mut cfg = serde_json::from_str::<GlobalConfig>(&cfg_string)?;
        cfg.generic_hero_xdb = path.join("Hero.(AdvMapHeroShared).xdb");
        cfg.generic_icon_128 = path.join("Icon.xdb");
        cfg.generic_icon_dds = path.join("Icon.dds");
        cfg.generic_map_xdb = path.join("map.xdb");
        cfg.session_configs_path = path.join("sessions\\");
        cfg.auth_path = path.join("auth\\client-secret.json");
        Ok(cfg)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeData {
    pub current_selected_map: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuntimeConfig {
    pub current_selected_map: i32,
    pub current_map_data: MapData,
}

impl RuntimeConfig {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let runtime_cfg_string = std::fs::read_to_string(path.join("runtime.json"))?;
        let runtime_data: RuntimeData = serde_json::from_str(&runtime_cfg_string)?;
        let current_map_string = std::fs::read_to_string(path.join("current_map_data.json"))?;
        let current_map_data: MapData = serde_json::from_str(&current_map_string)?;

        Ok(RuntimeConfig {
            current_selected_map: runtime_data.current_selected_map,
            current_map_data,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiersData {
    pub quests_generation_queue: Vec<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiersConfig {
    path: PathBuf,
    pub data: ModifiersData,
}

impl ModifiersConfig {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let path = path.join("modifiers.json");
        let data_string = std::fs::read_to_string(&path)?;
        Ok(ModifiersConfig {
            path,
            data: serde_json::from_str(&data_string)?,
        })
    }

    pub fn update(&self) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(&self.data)?;
        let mut file = std::fs::File::create(&self.path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataContainer {
    pub banks: Vec<BankConfigEntity>,
    pub buildings: Vec<BuildingConfigEntity>,
    pub artifacts: Vec<ArtifactConfigEntity>,
}

impl DataContainer {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let data_string = std::fs::read_to_string(path.join("objects_data.json"))?;
        Ok(serde_json::from_str(&data_string)?)
    }
}

#[derive(Debug)]
pub struct LocalAppManager {
    pub base_config: RwLock<GlobalConfig>,
    pub runtime_config: RwLock<RuntimeConfig>,
    pub modifiers_config: RwLock<ModifiersConfig>,
    pub current_profile_data: RwLock<ProfileConfig>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RepackerPathsData {
    pub from: PathBuf,
    pub to: PathBuf,
    pub last_update: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepackerFrontendData {
    pub label: String,
    pub update_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapFrontendModel {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineMessage {
    pub timestamp: String,
    pub message: String
}