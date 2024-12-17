use std::io::Write;
use std::path::PathBuf;

use homm5_repacker::Repacker;
use homm5_scaner::ScanExecutor;
use map_modifier::{quest, GenerateBoilerplate};
use map_modifier::quest::{QuestCreationRequest, QuestProgress};
use runtime_main::RuntimeRunner;
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
        mission_number: map.mission as u32
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
    initial: bool
) -> Result<(), ()> {
    app.dialog()
        .file()
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
pub async fn load_progress(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid,
    number: u32
) -> Result<String, ()> {
    let pool = app_manager.db_pool.read().await;
    if let Ok::<Option<QuestProgressDBModel>, _>(existing_progress) = 
        sqlx::query_as("SELECT * FROM progresses WHERE quest_id=? AND number=?")
            .bind(quest_id)
            .bind(number)
            .fetch_optional(&*pool)
            .await {
        if let Some(progress) = existing_progress {
            return Ok(progress.text.clone());
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
                    return Ok(String::new().clone());
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
pub async fn generate_quest(
    app_manager: State<'_, LocalAppManager>,
    quest_id: Uuid
) -> Result<(), ()> {

    let pool = app_manager.db_pool.read().await;
    let quest_data: QuestDBModel = sqlx::query_as(r#"
            SELECT * FROM quests WHERE id=?
        "#)
        .bind(quest_id)
        .fetch_one(&*pool)
        .await
        .unwrap();

    let progresses_data: Vec<QuestProgressDBModel> = sqlx::query_as(r#"
            SELECT * FROM progresses WHERE quest_id=?
        "#)
        .bind(quest_id)
        .fetch_all(&*pool)
        .await
        .unwrap();

    let quest_generator_request = QuestCreationRequest::new(PathBuf::from(quest_data.directory), quest_data.script_name)
        .with_name(quest_data.name)
        .with_desc(quest_data.desc)
        .with_progresses(progresses_data.into_iter().map(|p| { QuestProgress {number: p.number, text: p.text} }).collect())
        .with_mission_data(quest_data.campaign_number as u8, quest_data.mission_number as u8)
        .secondary(false)
        .initialy_active(false);

    let quest = quest_generator_request.generate();
    map_modifier::quest::test_convert(quest);

    Ok(())
}