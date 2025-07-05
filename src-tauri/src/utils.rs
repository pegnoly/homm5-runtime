use std::{collections::HashMap, io::Write, path::PathBuf};
use map_modifier::{artifacts::ArtifactConfigEntity, buildings::{BankConfigEntity, BuildingConfigEntity}, Map, MapData};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub generic_hero_xdb: Option<String>,
    pub generic_icon_128: Option<String>,
    pub generic_icon_dds: Option<String>,
    pub session_configs_path: Option<String>,
    pub exe_name: String,
    pub bin_path: String,
    pub data_path: String,
    pub mod_path: String,
    pub texts_path: String,
    pub repackers: HashMap<String, RepackerPathsData>,
    pub maps: Vec<Map>,
}

impl GlobalConfig {
    pub fn new(path: &PathBuf) -> Result<Self, Error> {
        let cfg_string = std::fs::read_to_string(path.join("main.json"))?;
        let mut cfg = serde_json::from_str::<GlobalConfig>(&cfg_string)?;
        if !cfg.generic_hero_xdb.is_some() {
            cfg.generic_hero_xdb = Some(path.join("Hero.(AdvMapHeroShared).xdb").to_string_lossy().to_string());
        }
        if !cfg.generic_icon_128.is_some() {
            cfg.generic_icon_128 = Some(path.join("Icon.xdb").to_string_lossy().to_string());
        }
        if !cfg.generic_icon_dds.is_some() {
            cfg.generic_icon_dds = Some(path.join("Icon.dds").to_string_lossy().to_string());
        }
        if !cfg.session_configs_path.is_some() {
            cfg.session_configs_path = Some(path.join("sessions\\").to_string_lossy().to_string());
        }
        Ok(cfg)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeData {
    pub current_selected_map: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuntimeConfig {
    pub current_selected_map: Option<u16>,
    pub current_map_data: MapData,
}

impl RuntimeConfig {
    pub fn new(path: &PathBuf) -> Result<Self, Error> {
        let runtime_cfg_string = std::fs::read_to_string(path.join("runtime.json"))?;
        let runtime_data: RuntimeData = serde_json::from_str(&runtime_cfg_string)?;
        let current_map_string = std::fs::read_to_string(path.join("current_map_data.json"))?;
        let current_map_data: MapData = serde_json::from_str(&current_map_string)?;

        Ok(RuntimeConfig {
            current_selected_map: Some(runtime_data.current_selected_map),
            current_map_data,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiersData {
    pub quests_generation_queue: Vec<i32>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiersConfig {
    path: PathBuf,
    pub data: ModifiersData
}

impl ModifiersConfig {
    pub fn new(path: &PathBuf) -> Result<Self, Error> {
        let path = path.join("modifiers.json");
        let data_string = std::fs::read_to_string(&path)?;
        Ok(ModifiersConfig { path: path, data: serde_json::from_str(&data_string)? })
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
    pub fn new(path: &PathBuf) -> Result<Self, Error> {   
        let data_string = std::fs::read_to_string(path.join("objects_data.json"))?;
        Ok(serde_json::from_str(&data_string)?)
    }
}

#[derive(Debug)]
pub struct LocalAppManager {
    pub base_config: RwLock<GlobalConfig>,
    pub runtime_config: RwLock<RuntimeConfig>,
    pub modifiers_config: RwLock<ModifiersConfig>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RepackerPathsData {
    pub from: String,
    pub to: String,
    pub last_update: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepackerFrontendData {
    pub label: String,
    pub update_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapFrontendModel {
    pub id: u16,
    pub name: String,
}
