use std::io::Write;
use std::path::PathBuf;

use homm5_repacker::Repacker;
use homm5_scaner::ScanExecutor;
use map_modifier::quest::{QuestBoilerplateHelper, QuestCreationRequest, QuestProgress};
use map_modifier::{GenerateBoilerplate, ModifiersQueue};
use runtime_main::RuntimeRunner;
use serde::{Serialize, Serializer};
use tauri::State;

use crate::services::QuestService;
use crate::utils::{Config, LocalAppManager, MapFrontendModel};

#[tauri::command]
pub async fn execute_scan(config: State<'_, Config>) -> Result<(), ()> {
    let data_path = PathBuf::from(&config.data_path);
    let root_folder = data_path.parent().unwrap();
    let maps_path = root_folder.join("Maps\\");
    let mods_path = root_folder.join("UserMODs\\");
    let output_path = data_path.join("MCCS_GeneratedFiles.pak");
    let scan_executor = ScanExecutor::new(output_path, vec![data_path, maps_path, mods_path]);
    scan_executor.run().await;
    Ok(())
}

#[tauri::command]
pub async fn run_game(config: State<'_, Config>) -> Result<(), ()> {
    let bin_path = &config.bin_path;
    let mut runtime_runner = RuntimeRunner::new(PathBuf::from(bin_path));
    runtime_runner.run();
    Ok(())
}

#[tauri::command]
pub async fn load_repackers(config: State<'_, Config>) -> Result<Vec<String>, ()> {
    let repackers_names = config
        .repackers
        .keys()
        .map(|r| r.clone())
        .collect::<Vec<String>>();
    Ok(repackers_names)
}

#[tauri::command]
pub async fn load_maps(config: State<'_, Config>) -> Result<Vec<MapFrontendModel>, ()> {
    Ok(config
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
    Ok(app_manager.runtime_config.lock().await.current_selected_map)
}

#[tauri::command]
pub async fn select_map(app_manager: State<'_, LocalAppManager>, id: u16) -> Result<(), ()> {
    let mut config_locked = app_manager.runtime_config.lock().await;
    config_locked.current_selected_map = Some(id);
    let exe_path = std::env::current_exe().unwrap();
    let runtime_cfg_path = exe_path.parent().unwrap().join("cfg\\runtime.json");
    let new_runtime_data = serde_json::to_string_pretty(&*config_locked).unwrap();
    let mut file = std::fs::File::create(&runtime_cfg_path).unwrap();
    file.write_all(&new_runtime_data.as_bytes()).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn repack(config: State<'_, Config>, repacker_label: String) -> Result<(), ()> {
    if let Some(repacker) = config.repackers.get(&repacker_label) {
        let from = PathBuf::from(&repacker.from);
        let to = PathBuf::from(&repacker.to);
        let repacker = Repacker::new(from, to);
        repacker.run();
    }
    Ok(())
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
    config: State<'_, Config>,
    app_manager: State<'_, LocalAppManager>,
    quest_service: State<'_, QuestService>,
) -> Result<(), super::error::Error> {
    let current_map_id = app_manager
        .runtime_config
        .lock()
        .await
        .current_selected_map
        .unwrap();
    let map = config.maps.iter().find(|m| m.id == current_map_id).unwrap();
    let mod_path = &config.mod_path;

    let mut modifiers_queue = ModifiersQueue {
        primary_quests: vec![],
        secondary_quests: vec![],
    };

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
            texts_path: config.texts_path.clone(),
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

    modifiers_queue.apply_changes_to_map(map);
    quest_service
        .delete_quests_from_queue(current_map_id)
        .await?;

    Ok(())
}
