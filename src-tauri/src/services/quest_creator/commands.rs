use std::path::PathBuf;

use map_modifier::quest::write_quest_text_file;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::{
    services::quest_creator::data::QuestProgressFrontendModel,
    utils::{Config, LocalAppManager},
};

use super::{data::QuestLoadingModel, service::QuestService};

#[tauri::command]
pub async fn collect_quests_for_selection(
    app_manager: State<'_, LocalAppManager>,
    quest_service: State<'_, QuestService>,
) -> Result<Vec<QuestLoadingModel>, ()> {
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();

    let base_config_locked = app_manager.base_config.read().await;
    let current_map = base_config_locked.maps.iter().find(|m| m.id == current_map_id).unwrap();

    match quest_service
        .get_quests_by_mission_data(current_map.campaign as u32, current_map.mission as u32)
        .await
    {
        Ok(quests) => Ok(quests
            .into_iter()
            .map(|q| QuestLoadingModel::from(q))
            .collect()),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_quest_name(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
) -> Result<String, ()> {
    match quest_service.get_quest_name(quest_id).await {
        Ok(name) => Ok(name),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_quest_desc(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
) -> Result<String, ()> {
    match quest_service.get_quest_desc(quest_id).await {
        Ok(desc) => Ok(desc),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_quest_script_name(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
) -> Result<String, ()> {
    match quest_service.get_quest_script_name(quest_id).await {
        Ok(script_name) => Ok(script_name),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_quest_directory(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
) -> Result<String, ()> {
    match quest_service.get_quest_directory(quest_id).await {
        Ok(directory) => Ok(directory),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_quest_is_secondary(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
) -> Result<bool, ()> {
    match quest_service.is_secondary_quest(quest_id).await {
        Ok(is_secondary) => Ok(is_secondary),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_quest_is_active(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
) -> Result<bool, ()> {
    match quest_service.is_active_quest(quest_id).await {
        Ok(is_active) => Ok(is_active),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn create_quest(
    app_manager: State<'_, LocalAppManager>,
    quest_service: State<'_, QuestService>,
    directory: String,
    script_name: String,
    name: String,
) -> Result<Uuid, ()> {
    let runtime_config_locked = app_manager.runtime_config.read().await;
    let base_config_locked = app_manager.base_config.read().await;
    let map = base_config_locked
        .maps
        .iter()
        .find(|m| m.id == runtime_config_locked.current_selected_map.unwrap())
        .unwrap();

    match quest_service
        .create_quest(
            &directory,
            &script_name,
            &name,
            map.campaign as u32,
            map.mission as u32,
        )
        .await
    {
        Ok(id) => Ok(id),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn pick_quest_directory(
    app: AppHandle,
    app_manager: State<'_, LocalAppManager>,
    initial: bool,
) -> Result<(), ()> {
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();

    let base_config_locked = app_manager.base_config.read().await;
    let map = base_config_locked.maps.iter().find(|m| m.id == current_map_id).unwrap();

    app.dialog()
        .file()
        .set_directory(PathBuf::from(&map.data_path))
        .set_can_create_directories(true)
        .pick_folder(move |f| {
            if initial == true {
                app.emit("quest_directory_picked", f.unwrap().to_string())
                    .unwrap();
            } else {
                app.emit("quest_directory_updated", f.unwrap().to_string())
                    .unwrap();
            }
        });

    Ok(())
}

#[tauri::command]
pub async fn update_quest_directory(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    directory: String,
) -> Result<(), ()> {
    match quest_service
        .update_quest_directory(quest_id, &directory)
        .await
    {
        Ok(()) => Ok(()),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn update_quest_script_name(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    script_name: String,
) -> Result<(), ()> {
    match quest_service
        .update_quest_script_name(quest_id, &script_name)
        .await
    {
        Ok(()) => Ok(()),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn update_quest_name(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    name: String,
) -> Result<(), ()> {
    match quest_service.update_quest_name(quest_id, &name).await {
        Ok(()) => Ok(()),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn update_quest_desc(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    desc: String,
) -> Result<(), ()> {
    match quest_service.update_quest_desc(quest_id, &desc).await {
        Ok(()) => Ok(()),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn update_is_secondary(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    is_secondary: bool,
) -> Result<(), ()> {
    match quest_service
        .update_quest_is_secondary(quest_id, is_secondary)
        .await
    {
        Ok(()) => Ok(()),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn update_is_active(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    is_active: bool,
) -> Result<(), ()> {
    match quest_service
        .update_quest_is_active(quest_id, is_active)
        .await
    {
        Ok(()) => Ok(()),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn load_progress(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    number: u32,
) -> Result<QuestProgressFrontendModel, ()> {
    match quest_service.get_quest_progress(quest_id, number).await {
        Ok(progress) => Ok(QuestProgressFrontendModel::from(progress)),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn save_progress(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    number: u32,
    text: String,
) -> Result<(), ()> {
    match quest_service.save_progress(quest_id, number, &text).await {
        Ok(_) => Ok(()),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn update_progress_concatenation(
    quest_service: State<'_, QuestService>,
    quest_id: Uuid,
    number: u32,
    concatenate: bool,
) -> Result<(), ()> {
    match quest_service
        .update_progress_concatenation(quest_id, number, concatenate)
        .await
    {
        Ok(_) => Ok(()),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn save_quest_text(
    config: State<'_, Config>,
    quest_directory: String,
    text_name: String,
    text: String,
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
    quest_id: Uuid,
) -> Result<(), ()> {
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();

    match quest_service
        .add_quest_to_queue(quest_id, current_map_id)
        .await
    {
        Ok(_) => Ok(()),
        _ => Err(()),
    }
}
