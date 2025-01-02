use std::io::Write;
use std::path::PathBuf;

use homm5_repacker::Repacker;
use homm5_scaner::ScanExecutor;
use map_modifier::{GenerateBoilerplate, ModifiersQueue};
use map_modifier::quest::{write_quest_text_file, QuestBoilerplateHelper, QuestCreationRequest, QuestProgress};
use runtime_main::RuntimeRunner;
use serde::{Serialize, Serializer};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::services::{QuestLoadingModel, QuestProgressFrontendModel, QuestService};
use crate::utils::{Config, LocalAppManager, MapFrontendModel};

#[tauri::command]
pub async fn execute_scan(
    config: State<'_, Config>
) -> Result<(), ()> {
    let data_path = &config.data_path;
    let scan_executor = ScanExecutor::new(PathBuf::from(data_path));
    scan_executor.run().await;
    Ok(())
}

#[tauri::command]
pub async fn run_game(
    config: State<'_, Config>
) -> Result<(), ()> {
    let bin_path = &config.bin_path;
    let mut runtime_runner = RuntimeRunner::new(PathBuf::from(bin_path));
    runtime_runner.run();
    Ok(())
}

#[tauri::command]
pub async fn load_repackers(
    config: State<'_, Config>
) -> Result<Vec<String>, ()> {
    let repackers_names = config.repackers.keys().map(|r| r.clone()).collect::<Vec<String>>();
    Ok(repackers_names)
}

#[tauri::command]
pub async fn load_maps(
    config: State<'_, Config>
) -> Result<Vec<MapFrontendModel>, ()> {
    Ok(config.maps.iter().map(|m| {
        MapFrontendModel {
            id: m.id,
            name: m.name.clone()
        }
    }).collect())
}

#[tauri::command]
pub async fn load_current_map(
    app_manager: State<'_, LocalAppManager>
) -> Result<Option<u16>, ()> {
    Ok(app_manager.runtime_config.lock().await.current_selected_map)
}

#[tauri::command]
pub async fn select_map(
    app_manager: State<'_, LocalAppManager>,
    id: u16
) -> Result<(), ()> {
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
pub async fn repack(
    config: State<'_, Config>,
    repacker_label: String
) -> Result<(), ()> {
    if let Some(repacker) = config.repackers.get(&repacker_label) {
        let from = PathBuf::from(&repacker.from);
        let to = PathBuf::from(&repacker.to);
        let repacker = Repacker::new(from, to);
        repacker.run();
    }
    Ok(())
}

#[tauri::command]
pub async fn collect_quests_for_selection(
    app_manager: State<'_, LocalAppManager>,
    config: State<'_, Config>,
    quest_service: State<'_, QuestService>
) -> Result<Vec<QuestLoadingModel>, ()> {

    let current_map_id = app_manager.runtime_config.lock().await.current_selected_map.unwrap();
    let current_map = config.maps.iter().find(|m| m.id == current_map_id).unwrap();

    match quest_service.get_quests_by_mission_data(current_map.campaign as u32, current_map.mission as u32).await {
        Ok(quests) => {
            Ok(quests.into_iter().map(|q| QuestLoadingModel::from(q)).collect())
        },
        Err(_error) => {
            Err(())
        }
    }
}

#[tauri::command]
pub async fn load_quest_name(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid
) -> Result<String, ()> {

    match quest_service.get_quest_name(quest_id).await {
        Ok(name) => {
            Ok(name)
        },
        Err(_error) => {
            Err(())
        }
    }
}

#[tauri::command]
pub async fn load_quest_desc(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid
) -> Result<String, ()> {
    match quest_service.get_quest_desc(quest_id).await {
        Ok(desc) => {
            Ok(desc)
        },
        Err(_error) => {
            Err(())
        }
    }
}

#[tauri::command]
pub async fn load_quest_script_name(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid
) -> Result<String, ()> {
    match quest_service.get_quest_script_name(quest_id).await {
        Ok(script_name) => {
            Ok(script_name)
        },
        Err(_error) => {
            Err(())
        }
    }
}

#[tauri::command]
pub async fn load_quest_directory(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid
) -> Result<String, ()> {
    match quest_service.get_quest_directory(quest_id).await {
        Ok(directory) => {
            Ok(directory)
        },
        Err(_error) => {
            Err(())
        }
    }
}

#[tauri::command]
pub async fn load_quest_is_secondary(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid
) -> Result<bool, ()> {
    match quest_service.is_secondary_quest(quest_id).await {
        Ok(is_secondary) => {
            Ok(is_secondary)
        },
        Err(_error) => {
            Err(())
        }
    }
}

#[tauri::command]
pub async fn load_quest_is_active(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid
) -> Result<bool, ()> {
    match quest_service.is_active_quest(quest_id).await {
        Ok(is_active) => {
            Ok(is_active)
        },
        Err(_error) => {
            Err(())
        }
    }
}

#[tauri::command]
pub async fn create_quest(
    config: State<'_, Config>,
    app_manager: State<'_, LocalAppManager>,
    quest_service: State<'_, QuestService>,
    directory: String,
    script_name: String,
    name: String
) -> Result<Uuid, ()> {
    let runtime_config = app_manager.runtime_config.lock().await;
    let map = config.maps.iter().find(|m| {
        m.id == runtime_config.current_selected_map.unwrap()
    }).unwrap();

    match quest_service.create_quest(&directory, &script_name, &name, map.campaign as u32, map.mission as u32).await {
        Ok(id) => {
            Ok(id)
        },
        Err(_error) => {
            Err(())
        }
    }
}

#[tauri::command]
pub async fn pick_quest_directory(
    app: AppHandle,
    config: State<'_, Config>,
    app_manager: State<'_, LocalAppManager>,
    initial: bool
) -> Result<(), ()> {
    let current_map_id = app_manager.runtime_config.lock().await.current_selected_map.unwrap();
    let map = config.maps.iter().find(|m| m.id == current_map_id).unwrap();

    app.dialog()
        .file()
        .set_directory(PathBuf::from(&map.data_path))
        .set_can_create_directories(true)
        .pick_folder(move |f| {
            if initial == true {
                app.emit("quest_directory_picked", f.unwrap().to_string()).unwrap();
            }
            else {
                app.emit("quest_directory_updated", f.unwrap().to_string()).unwrap();
            }
        });

    Ok(())
}

#[tauri::command]
pub async fn update_quest_directory(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    directory: String
) -> Result<(), ()> {
    if let Ok(()) = quest_service.update_quest_directory(quest_id, &directory).await {
        Ok(())
    }
    else {
        Err(())
    }
}

#[tauri::command]
pub async fn update_quest_script_name(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    script_name: String
) -> Result<(), ()> {
    if let Ok(()) = quest_service.update_quest_script_name(quest_id, &script_name).await {
        Ok(())
    }
    else {
        Err(())
    }
}

#[tauri::command]
pub async fn update_quest_name(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    name: String
) -> Result<(), ()> {
    if let Ok(()) = quest_service.update_quest_name(quest_id, &name).await {
        Ok(())
    }
    else {
        Err(())
    }
}

#[tauri::command]
pub async fn update_quest_desc(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    desc: String
) -> Result<(), ()> {
    if let Ok(()) = quest_service.update_quest_desc(quest_id, &desc).await {
        Ok(())
    }
    else {
        Err(())
    }
}

#[tauri::command]
pub async fn update_is_secondary(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    is_secondary: bool
) -> Result<(), ()> {
    if let Ok(()) = quest_service.update_quest_is_secondary(quest_id, is_secondary).await {
        Ok(())
    }
    else {
        Err(())
    }
}

#[tauri::command]
pub async fn update_is_active(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    is_active: bool
) -> Result<(), ()> {
    if let Ok(()) = quest_service.update_quest_is_active(quest_id, is_active).await {
        Ok(())
    }
    else {
        Err(())
    }
}

#[tauri::command]
pub async fn load_progress(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    number: u32
) -> Result<QuestProgressFrontendModel, ()> {
    if let Ok(progress) = quest_service.get_quest_progress(quest_id, number).await {
        Ok(QuestProgressFrontendModel::from(progress))
    }
    else {
        Err(())
    }
}

#[tauri::command]
pub async fn save_progress(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    number: u32,
    text: String
) -> Result<(), ()> {
    if let Ok(_) = quest_service.save_progress(quest_id, number, &text).await {
        Ok(())
    }
    else {
        Err(())
    }
}

#[tauri::command]
pub async fn update_progress_concatenation(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    number: u32,
    concatenate: bool
) -> Result<(), ()> {
    if let Ok(_) = quest_service.update_progress_concatenation(quest_id, number, concatenate).await {
        Ok(())
    }
    else {
        Err(())
    }
}

#[tauri::command]
pub async fn save_quest_text(
    config: State<'_, Config>,
    quest_directory: String,
    text_name: String,
    text: String
) -> Result<(), ()> {
    let dir_relative_path = quest_directory.replace(&config.mod_path, "");
    println!("Directory relative path: {}", &dir_relative_path);
    let text_directory = format!("{}{}\\Texts\\", &config.texts_path, &dir_relative_path);
    std::fs::create_dir_all(&text_directory).unwrap();
    let mut file = std::fs::File::create(format!("{}{}.txt", &text_directory, &text_name)).unwrap();
    write_quest_text_file(&mut file, text);
    Ok(())
}

// This must just push quest id into db for modifications.
#[tauri::command]
pub async fn add_quest_to_queue(
    app_manager: State<'_, LocalAppManager>,
    quest_service: State<'_, QuestService>,
    quest_id: Uuid
) -> Result<(), ()> {
    let current_map_id = app_manager.runtime_config.lock().await.current_selected_map.unwrap();

    if let Ok(_) = quest_service.add_quest_to_queue(quest_id, current_map_id).await {
        Ok(())
    }
    else {
        Err(())
    }

}

#[derive(Debug, thiserror::Error)]
pub enum ModificationsError {
    #[error(transparent)]
    SqlxError(#[from]sqlx::Error)
}

impl Serialize for ModificationsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer 
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
pub async fn apply_modifications(
    config: State<'_, Config>,
    app_manager: State<'_, LocalAppManager>,
    quest_service: State<'_, QuestService>
) -> Result<(), ModificationsError> {
    let current_map_id = app_manager.runtime_config.lock().await.current_selected_map.unwrap();
    let map = config.maps.iter().find(|m| m.id == current_map_id).unwrap();
    let mod_path = &config.mod_path;

    let mut modifiers_queue = ModifiersQueue { primary_quests: vec![], secondary_quests: vec![] };

    // get all ids of quest for current map

    // get all quests data for these ids and convert db models to quests
    
    let models = quest_service.get_quests_to_apply(current_map_id).await?;

    for model in models {
        let progresses = quest_service.get_quest_progresses(model.id).await?;
        let request = QuestCreationRequest::new(PathBuf::from(model.directory), model.script_name)
            .with_name(model.name)
            .with_desc(model.desc)
            .with_progresses(progresses.into_iter().map(|p| { QuestProgress {number: p.number, text: p.text, concatenate: p.concatenate} }).collect())
            .with_mission_data(model.campaign_number as u8, model.mission_number as u8)
            .secondary(model.is_secondary)
            .initialy_active(model.is_active);

        let quest = request.generate(Some(&QuestBoilerplateHelper {
            mod_path: mod_path.clone(), 
            map_data_path: map.data_path.clone(), 
            texts_path: config.texts_path.clone()
        }));
        if model.is_secondary {
            modifiers_queue.secondary_quests.push(quest);
        }
        else {
            modifiers_queue.primary_quests.push(quest);
        }
    }

    println!("Primary quests: {:?}", &modifiers_queue.primary_quests);
    println!("Secondary quests: {:?}", &modifiers_queue.secondary_quests);

    modifiers_queue.apply_changes_to_map(map);
    quest_service.delete_quests_from_queue(current_map_id).await?;

    Ok(())
}