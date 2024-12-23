use std::io::Write;
use std::path::PathBuf;

use derive_more::derive::From;
use homm5_repacker::Repacker;
use homm5_scaner::ScanExecutor;
use map_modifier::{quest, FileRef, GenerateBoilerplate, ModifiersQueue, Quest};
use map_modifier::quest::{QuestCreationRequest, QuestProgress};
use runtime_main::RuntimeRunner;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::Error;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::utils::{Config, LocalAppManager, MapFrontendModel, QuestDBModel, QuestFrontendModel, QuestProgressDBModel, RuntimeConfig};

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
pub async fn create_quest(
    config: State<'_, Config>,
    app_manager: State<'_, LocalAppManager>,
    directory: String,
    script_name: String,
    name: String
) -> Result<QuestFrontendModel, ()> {
    let pool = app_manager.db_pool.read().await;
    let runtime_config = app_manager.runtime_config.lock().await;
    let map = config.maps.iter().find(|m| {
        m.id == runtime_config.current_selected_map.unwrap()
    }).unwrap();
    let quest = QuestDBModel {
        id: Uuid::new_v4(),
        directory: directory,
        name: name,
        desc: String::new(),
        script_name: script_name,
        campaign_number: map.campaign as u32,
        mission_number: map.mission as u32,
        is_active: false,
        is_secondary: false,
        is_first_init: false
    };
    let quest_insert_result = sqlx::query(r#"
        INSERT INTO quests (id, directory, campaign_number, mission_number, name, desc, script_name) VALUES (?, ?, ?, ?, ?, ?, ?)
    "#)
        .bind(&quest.id)
        .bind(&quest.directory)
        .bind(&quest.campaign_number)
        .bind(&quest.mission_number)
        .bind(&quest.name)
        .bind(&quest.desc)
        .bind(&quest.script_name)
        .execute(&*pool)
        .await;

    match quest_insert_result {
        Ok(_) => {
            Ok(QuestFrontendModel::from(quest))
        },
        Err(insert_failure) => {
            println!("Failed to create new quest: {}", insert_failure.to_string());
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
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid,
    directory: String
) -> Result<(), ()> {
    let pool = app_manager.db_pool.read().await;
    let directory_update_result: Result<QuestDBModel, Error> = sqlx::query_as(r#"
            UPDATE quests 
            SET directory=?
            WHERE id=?
            RETURNING *;
        "#)
        .bind(directory)
        .bind(quest_id)
        .fetch_one(&*pool)
        .await;

    match directory_update_result {
        Ok(_) => {
            Ok(())
        },
        Err(update_failure) => {
            println!("Failed to update quest directory: {}", update_failure.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn update_quest_script_name(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid,
    script_name: String
) -> Result<(), ()> {
    let pool = app_manager.db_pool.read().await;
    let script_name_update_result: Result<QuestDBModel, Error> = sqlx::query_as(r#"
            UPDATE quests 
            SET script_name=?
            WHERE id=?
            RETURNING *;
        "#)
        .bind(script_name)
        .bind(quest_id)
        .fetch_one(&*pool)
        .await;

    match script_name_update_result {
        Ok(_) => {
            Ok(())
        },
        Err(update_failure) => {
            println!("Failed to update quest script name: {}", update_failure.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn update_quest_name(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid,
    name: String
) -> Result<(), ()> {
    let pool = app_manager.db_pool.read().await;
    let name_update_result: Result<QuestDBModel, Error> = sqlx::query_as(r#"
            UPDATE quests 
            SET name=?
            WHERE id=?
            RETURNING *;
        "#)
        .bind(name)
        .bind(quest_id)
        .fetch_one(&*pool)
        .await;

    match name_update_result {
        Ok(_) => {
            Ok(())
        },
        Err(update_failure) => {
            println!("Failed to update quest name: {}", update_failure.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn update_quest_desc(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid,
    desc: String
) -> Result<(), ()> {
    let pool = app_manager.db_pool.read().await;
    let desc_update_result: Result<QuestDBModel, Error> = sqlx::query_as(r#"
            UPDATE quests 
            SET desc=?
            WHERE id=?
            RETURNING *;
        "#)
        .bind(desc)
        .bind(quest_id)
        .fetch_one(&*pool)
        .await;

    match desc_update_result {
        Ok(_) => {
            Ok(())
        },
        Err(update_failure) => {
            println!("Failed to update quest desc: {}", update_failure.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn update_is_secondary(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid,
    is_secondary: bool
) -> Result<(), ()> {
    let pool = app_manager.db_pool.read().await;
    let update_result: Result<QuestDBModel, Error> = sqlx::query_as(r#"
            UPDATE quests 
            SET is_secondary=?
            WHERE id=?
            RETURNING *;
        "#)
        .bind(is_secondary)
        .bind(quest_id)
        .fetch_one(&*pool)
        .await;

    match update_result {
        Ok(_) => Ok(()),
        Err(update_failure) => {
            println!("Failed to update is_secondary quest param: {}", update_failure.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn update_is_active(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid,
    is_active: bool
) -> Result<(), ()> {
    let pool = app_manager.db_pool.read().await;
    let update_result: Result<QuestDBModel, Error> = sqlx::query_as(r#"
            UPDATE quests 
            SET is_active=?
            WHERE id=?
            RETURNING *;
        "#)
        .bind(is_active)
        .bind(quest_id)
        .fetch_one(&*pool)
        .await;

    match update_result {
        Ok(_) => Ok(()),
        Err(update_failure) => {
            println!("Failed to update is_active quest param: {}", update_failure.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn load_progress(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid,
    number: u32
) -> Result<QuestProgressFrontendModel, ()> {
    let pool = app_manager.db_pool.read().await;
    if let Ok::<Option<QuestProgressDBModel>, _>(existing_progress) = 
        sqlx::query_as("SELECT * FROM progresses WHERE quest_id=? AND number=?")
            .bind(quest_id)
            .bind(number)
            .fetch_optional(&*pool)
            .await {
        if let Some(progress) = existing_progress {
            return Ok(QuestProgressFrontendModel {
                text: progress.text,
                concatenate: progress.concatenate
            });
        }
        else {
            let progress_id = Uuid::new_v4();
            let progress_creation_result = sqlx::query(r#"
                    INSERT INTO progresses (id, quest_id, number, text) VALUES (?, ?, ?, ?) 
                "#)
                .bind(progress_id)
                .bind(quest_id)
                .bind(number)
                .bind(String::new())
                .execute(&*pool)
                .await;
            match progress_creation_result {
                Ok(_creation_success) => {
                    return Ok(QuestProgressFrontendModel {text: String::new(), concatenate: true});
                }
                Err(creation_failure) => {
                    println!("Failed to create quest progress: {}", creation_failure.to_string());
                    return Err(())
                }
            }
        }
    }
    else {
        return Err(());
    } 
}

#[tauri::command]
pub async fn save_progress(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid,
    number: u32,
    text: String
) -> Result<(), ()> {
    let pool = app_manager.db_pool.read().await;
    let progress_update_result: Result<QuestProgressDBModel, Error> = sqlx::query_as(r#"
            UPDATE progresses 
            SET text=?
            WHERE quest_id=? AND number=?
            RETURNING *;
        "#)
        .bind(text)
        .bind(quest_id)
        .bind(number)
        .fetch_one(&*pool)
        .await;

    match progress_update_result {
        Ok(_) => {
            Ok(())
        },
        Err(update_failure) => {
            println!("Failed to update progress: {}", update_failure.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn update_progress_concatenation(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid,
    number: u32,
    concatenate: bool
) -> Result<(), ()> {
    let pool = app_manager.db_pool.read().await;
    let _update_result: Result<QuestProgressDBModel, Error> = sqlx::query_as(r#"
            UPDATE progresses
            SET concatenate=?
            WHERE quest_id=? AND number=?
            RETURNING *;
        "#)
        .bind(concatenate)
        .bind(quest_id)
        .bind(number)
        .fetch_one(&*pool)
        .await;

    Ok(())
}

// This must just push quest id into db for modifications.
#[tauri::command]
pub async fn add_quest_to_queue(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid
) -> Result<(), ()> {
    let pool = app_manager.db_pool.read().await;
    let current_map_id = app_manager.runtime_config.lock().await.current_selected_map.unwrap();

    let quest_add_result = sqlx::query(r#"
            INSERT INTO quest_modifiers (quest_id, map_id) VALUES (?, ?)
            ON CONFLICT
            DO NOTHING;
        "#)
        .bind(quest_id)
        .bind(current_map_id)
        .execute(&*pool)
        .await;

    match quest_add_result {
        Ok(_) => {
            Ok(())
        },
        Err(add_failure) => {
            println!("Failed to add quest to queue: {}", add_failure);
            Err(())
        }
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
) -> Result<(), ModificationsError> {
    let pool = app_manager.db_pool.read().await;
    let current_map_id = app_manager.runtime_config.lock().await.current_selected_map.unwrap();
    let map = config.maps.iter().find(|m| m.id == current_map_id).unwrap();
    let mod_path = &config.mod_path;

    let mut modifiers_queue = ModifiersQueue { primary_quests: vec![], secondary_quests: vec![] };

    // get all ids of quest for current map

    // get all quests data for these ids and convert db models to quests
    
    let quests_models: Vec<QuestDBModel> = sqlx::query_as(r#"
            SELECT * FROM quests 
            WHERE id IN
            (SELECT quest_id FROM quest_modifiers WHERE map_id=?)
        "#)
        .bind(current_map_id)
        .fetch_all(&*pool)
        .await?;

    for model in quests_models {
        let progresses: Vec<QuestProgressDBModel> = sqlx::query_as(r#"
            SELECT * FROM progresses WHERE quest_id=?
        "#)
        .bind(model.id)
        .fetch_all(&*pool)
        .await?;

        let request = QuestCreationRequest::new(PathBuf::from(model.directory), model.script_name)
            .with_name(model.name)
            .with_desc(model.desc)
            .with_progresses(progresses.into_iter().map(|p| { QuestProgress {number: p.number, text: p.text, concatenate: p.concatenate} }).collect())
            .with_mission_data(model.campaign_number as u8, model.mission_number as u8)
            .secondary(model.is_secondary)
            .initialy_active(model.is_active);

        let quest = request.generate(Some(mod_path));
        if model.is_secondary {
            modifiers_queue.secondary_quests.push(quest);
        }
        else {
            modifiers_queue.primary_quests.push(quest);
        }
    }

    modifiers_queue.apply_changes_to_map(map);

    Ok(())
}