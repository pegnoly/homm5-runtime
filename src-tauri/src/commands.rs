use std::io::Write;
use std::path::PathBuf;

use homm5_repacker::Repacker;
use homm5_scaner::prelude::ScanerService;
use itertools::Itertools;
use map_modifier::quest::{QuestBoilerplateHelper, QuestCreationRequest, QuestProgress};
use map_modifier::{GenerateBoilerplate, MapData, ModifiersQueue};
use runtime_main::RuntimeRunner;
use serde::{Serialize, Serializer};
use tauri::State;

use crate::services::QuestService;
use crate::utils::{LocalAppManager, MapFrontendModel, RepackerFrontendData};
use crate::{DataContainer, RuntimeData};

#[tauri::command]
pub async fn execute_scan(
    app_manager: State<'_, LocalAppManager>,
    scaner_service: State<'_, ScanerService>,
) -> Result<(), ()> {
    let base_config_locked = app_manager.base_config.read().await;
    let data_path = PathBuf::from(&base_config_locked.data_path);
    let root_folder = data_path.parent().unwrap();
    let maps_path = root_folder.join("Maps\\");
    let mods_path = root_folder.join("UserMODs\\");
    let output_path = data_path.join("MCCS_GeneratedFiles.pak");
    scaner_service
        .run(vec![data_path, maps_path, mods_path], output_path)
        .await;
    Ok(())
}

#[tauri::command]
pub async fn run_game(app_manager: State<'_, LocalAppManager>) -> Result<(), ()> {
    let base_config_locked = app_manager.base_config.read().await;
    let bin_path = &base_config_locked.bin_path;
    let mut runtime_runner = RuntimeRunner::new(PathBuf::from(bin_path));
    runtime_runner.run();
    Ok(())
}

#[tauri::command]
pub async fn load_repackers(
    app_manager: State<'_, LocalAppManager>,
) -> Result<Vec<RepackerFrontendData>, ()> {
    let base_config_locked = app_manager.base_config.read().await;
    let repackers_data = base_config_locked
        .repackers
        .iter()
        .map(|(key, value)| RepackerFrontendData {
            label: key.clone(),
            update_time: value.last_update.clone(),
        })
        .collect_vec();
    Ok(repackers_data)
}

#[tauri::command]
pub async fn load_maps(
    app_manager: State<'_, LocalAppManager>,
) -> Result<Vec<MapFrontendModel>, ()> {
    let base_config_locked = app_manager.base_config.read().await;
    Ok(base_config_locked
        .maps
        .iter()
        .map(|m| MapFrontendModel {
            id: m.id,
            name: m.name.clone(),
        })
        .collect())
}

#[tauri::command]
pub async fn load_current_map(app_manager: State<'_, LocalAppManager>) -> Result<Option<u16>, ()> {
    Ok(app_manager.runtime_config.read().await.current_selected_map)
}

#[tauri::command]
pub async fn select_map(app_manager: State<'_, LocalAppManager>, id: u16) -> Result<(), ()> {
    let mut runtime_config_locked = app_manager.runtime_config.write().await;
    runtime_config_locked.current_selected_map = Some(id);
    let exe_path = std::env::current_exe().unwrap();
    let runtime_cfg_path = exe_path.parent().unwrap().join("cfg\\runtime.json");
    let new_runtime_data = serde_json::to_string_pretty(&RuntimeData {
        current_selected_map: id,
    })
    .unwrap();
    let mut file = std::fs::File::create(&runtime_cfg_path).unwrap();
    file.write_all(new_runtime_data.as_bytes()).unwrap();

    let base_config_locked = app_manager.base_config.read().await;
    let map = base_config_locked
        .maps
        .iter()
        .find(|m| m.id == runtime_config_locked.current_selected_map.unwrap())
        .unwrap();
    let current_map_data = MapData::read(map);
    let current_map_data_path = exe_path
        .parent()
        .unwrap()
        .join("cfg\\current_map_data.json");
    let current_map_data_string = serde_json::to_string_pretty(&current_map_data).unwrap();
    let mut file = std::fs::File::create(&current_map_data_path).unwrap();
    file.write_all(current_map_data_string.as_bytes()).unwrap();
    runtime_config_locked.current_map_data = current_map_data;
    Ok(())
}

#[tauri::command]
pub async fn repack(
    app_manager: State<'_, LocalAppManager>,
    repacker_label: String,
) -> Result<String, ()> {
    let mut base_config_locked = app_manager.base_config.write().await;
    if let Some(repacker_data) = base_config_locked.repackers.get_mut(&repacker_label) {
        let from = PathBuf::from(&repacker_data.from);
        let to = PathBuf::from(&repacker_data.to);
        let repacker = Repacker::new(from, to);
        repacker.run();
        let date = chrono::Local::now()
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
            .to_string();
        repacker_data.last_update = date.clone();
        let new_base_config_data = serde_json::to_string_pretty(&*base_config_locked).unwrap();
        let exe_path = std::env::current_exe().unwrap();
        let base_cfg_path = exe_path.parent().unwrap().join("cfg\\main.json");
        let mut file = std::fs::File::create(&base_cfg_path).unwrap();
        file.write_all(new_base_config_data.as_bytes()).unwrap();
        Ok(date)
    } else {
        Err(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ModificationsError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

impl Serialize for ModificationsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
pub async fn apply_modifications(
    app_manager: State<'_, LocalAppManager>,
    quest_service: State<'_, QuestService>,
    data_containter: State<'_, DataContainer>,
) -> Result<(), super::error::Error> {
    let mut runtime_config_locked = app_manager.runtime_config.write().await;
    let base_config_locked = app_manager.base_config.read().await;
    let current_map_id = runtime_config_locked.current_selected_map.unwrap();
    let map = base_config_locked
        .maps
        .iter()
        .find(|m| m.id == current_map_id)
        .unwrap();
    let mod_path = &base_config_locked.mod_path;

    let mut modifiers_queue = ModifiersQueue::new(
        &data_containter.banks,
        &data_containter.buildings,
        &data_containter.artifacts,
    );

    // get all ids of quest for current map

    // get all quests data for these ids and convert db models to quests

    let models = quest_service.get_quests_to_apply(current_map_id).await?;

    for model in models {
        let progresses = quest_service.get_quest_progresses(model.id).await?;
        let request = QuestCreationRequest::new(PathBuf::from(model.directory), model.script_name)
            .with_name(model.name)
            .with_desc(model.desc)
            .with_progresses(
                progresses
                    .into_iter()
                    .map(|p| QuestProgress {
                        number: p.number,
                        text: p.text,
                        concatenate: p.concatenate,
                    })
                    .collect(),
            )
            .with_mission_data(model.campaign_number as u8, model.mission_number as u8)
            .secondary(model.is_secondary)
            .initialy_active(model.is_active);

        if let Ok(quest) = request.generate(Some(&QuestBoilerplateHelper {
            mod_path: mod_path.clone(),
            map_data_path: map.data_path.clone(),
            texts_path: base_config_locked.texts_path.clone(),
        })) {
            if model.is_secondary {
                modifiers_queue.secondary_quests.push(quest);
            } else {
                modifiers_queue.primary_quests.push(quest);
            }
        }
    }

    println!("Primary quests: {:?}", &modifiers_queue.primary_quests);
    println!("Secondary quests: {:?}", &modifiers_queue.secondary_quests);

    modifiers_queue.apply_changes_to_map(map, &mut runtime_config_locked.current_map_data);
    quest_service
        .delete_quests_from_queue(current_map_id)
        .await?;

    Ok(())
}
