use std::path::PathBuf;

use editor_tools::prelude::{CreateQuestPayload, GetProgressPayload, QuestGeneratorRepo, QuestModel, QuestProgressModel, SaveProgressPayload, UpdateQuestPayload};
use map_modifier::quest::write_quest_text_file;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;

use crate::{error::Error, utils::{GlobalConfig, LocalAppManager}};

#[tauri::command]
pub async fn load_quests(
    app_manager: State<'_, LocalAppManager>,
    quests_repo: State<'_, QuestGeneratorRepo>
) -> Result<Vec<QuestModel>, Error> {
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();
    Ok(quests_repo.load_quests(current_map_id as i32).await?)
}

#[tauri::command]
pub async fn load_quest(
    quests_repo: State<'_, QuestGeneratorRepo>,
    id: i32
) -> Result<Option<QuestModel>, Error> {
    Ok(quests_repo.load_quest(id).await?)
}


#[tauri::command]
pub async fn create_quest(
    app_manager: State<'_, LocalAppManager>,
    quests_repo: State<'_, QuestGeneratorRepo>,
    directory: String,
    script_name: String,
    name: String,
) -> Result<QuestModel, Error> {
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();

    Ok(quests_repo.create_quest(CreateQuestPayload { mission_id: current_map_id as i32, name, script_name, directory }).await?)
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
    let map = base_config_locked
        .maps
        .iter()
        .find(|m| m.id == current_map_id)
        .unwrap();

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
    quests_repo: State<'_, QuestGeneratorRepo>,
    quest_id: i32,
    directory: String,
) -> Result<(), Error> {
    Ok(quests_repo.update_quest(UpdateQuestPayload::new(quest_id).with_directory(directory)).await?)
}

#[tauri::command]
pub async fn update_quest_script_name(
    quests_repo: State<'_, QuestGeneratorRepo>,
    quest_id: i32,
    script_name: String,
) -> Result<(), Error> {
    Ok(quests_repo.update_quest(UpdateQuestPayload::new(quest_id).with_script_name(script_name)).await?)
}

#[tauri::command]
pub async fn update_quest_name(
    quests_repo: State<'_, QuestGeneratorRepo>,
    quest_id: i32,
    name: String,
) -> Result<(), Error> {
    Ok(quests_repo.update_quest(UpdateQuestPayload::new(quest_id).with_name(name)).await?)
}

#[tauri::command]
pub async fn update_quest_desc(
    quests_repo: State<'_, QuestGeneratorRepo>,
    quest_id: i32,
    desc: String,
) -> Result<(), Error> {
    Ok(quests_repo.update_quest(UpdateQuestPayload::new(quest_id).with_desc(desc)).await?)
}

#[tauri::command]
pub async fn update_is_secondary(
    quests_repo: State<'_, QuestGeneratorRepo>,
    quest_id: i32,
    is_secondary: bool,
) -> Result<(), Error> {
    Ok(quests_repo.update_quest(UpdateQuestPayload::new(quest_id).with_secondary(is_secondary)).await?)
}

#[tauri::command]
pub async fn update_is_active(
    quests_repo: State<'_, QuestGeneratorRepo>,
    quest_id: i32,
    is_active: bool,
) -> Result<(), Error> {
    Ok(quests_repo.update_quest(UpdateQuestPayload::new(quest_id).with_active(is_active)).await?)
}

#[tauri::command]
pub async fn load_quest_progress(
    quests_repo: State<'_, QuestGeneratorRepo>,
    quest_id: i32,
    number: i32,
) -> Result<QuestProgressModel, Error> {
    if let Some(progress) = quests_repo.get_progress(GetProgressPayload { quest_id, number }).await? {
        Ok(progress)
    } else {
        Ok(quests_repo.create_progress(GetProgressPayload { quest_id, number }).await?)
    }
}

#[tauri::command]
pub async fn save_progress(
    quests_repo: State<'_, QuestGeneratorRepo>,
    id: i32,
    text: String,
    concatenate: bool
) -> Result<(), Error> {
    Ok(quests_repo.save_progress(SaveProgressPayload { id, text, concatenate }).await?)
}

#[tauri::command]
pub async fn save_quest_text(
    config: State<'_, GlobalConfig>,
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

#[tauri::command]
pub async fn add_quest_to_queue(
    app_manager: State<'_, LocalAppManager>,
    quest_id: i32,
) -> Result<(), Error> {
    let mut modifiers_config = app_manager.modifiers_config.write().await;
    if !modifiers_config.data.quests_generation_queue.contains(&quest_id) {
        modifiers_config.data.quests_generation_queue.push(quest_id);
        modifiers_config.update()?;
    }
    Ok(())
}