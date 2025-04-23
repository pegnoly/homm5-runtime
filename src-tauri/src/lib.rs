use editor_tools::services::banks::service::BanksService;
use map_modifier::{artifacts::ArtifactConfigEntity, buildings::{BankConfigEntity, BuildingConfigEntity}, MapData};
use serde::{Deserialize, Serialize};
use services::dialog_generator::prelude::*;
use services::quest_creator::prelude::*;

use tokio::sync::Mutex;
use utils::{Config, LocalAppManager, RuntimeConfig};

mod commands;
mod error;
mod services;
mod source;
mod utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeData {
    pub current_selected_map: u16
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataContainer {
    pub banks: Vec<BankConfigEntity>,
    pub buildings: Vec<BuildingConfigEntity>,
    pub artifacts: Vec<ArtifactConfigEntity>
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let exe_path = std::env::current_exe().unwrap();
    let cfg_path = exe_path.parent().unwrap().join("cfg\\");

    let cfg_string = std::fs::read_to_string(cfg_path.join("main.json")).unwrap();
    let cfg: Config = serde_json::from_str(&cfg_string).unwrap();

    let runtime_cfg_string = std::fs::read_to_string(cfg_path.join("runtime.json")).unwrap();
    let runtime_data: RuntimeData = serde_json::from_str(&runtime_cfg_string).unwrap();
    let current_map_string = std::fs::read_to_string(cfg_path.join("current_map_data.json")).unwrap();
    let current_map_data: MapData = serde_json::from_str(&current_map_string).unwrap();

    let runtime_config = RuntimeConfig {
        current_selected_map: Some(runtime_data.current_selected_map),
        current_map_data
    };

    let data_string = std::fs::read_to_string(cfg_path.join("objects_data.json")).unwrap();
    let data: DataContainer = serde_json::from_str(&data_string).unwrap();

    let db_path = cfg_path.join("runtime_database.db");
    if !db_path.exists() {
        std::fs::File::create(&db_path).unwrap();
    }

    let pool = sqlx::SqlitePool::connect(db_path.to_str().unwrap())
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let quest_service = QuestService::new(pool.clone());
    let dialog_generator_service = DialogGeneratorService::new(pool.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(cfg)
        .manage(LocalAppManager {
            runtime_config: Mutex::new(runtime_config),
        })
        .manage(quest_service)
        .manage(dialog_generator_service)
        .manage(BanksService::new(pool.clone()))
        .manage(data)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::execute_scan,
            commands::run_game,
            commands::load_repackers,
            commands::repack,
            commands::load_maps,
            commands::load_current_map,
            commands::select_map,
            commands::apply_modifications,
            // quest commands
            collect_quests_for_selection,
            pick_quest_directory,
            create_quest,
            save_progress,
            load_progress,
            update_progress_concatenation,
            update_quest_directory,
            update_quest_script_name,
            update_quest_name,
            update_quest_desc,
            update_is_secondary,
            update_is_active,
            load_quest_name,
            load_quest_desc,
            load_quest_directory,
            load_quest_script_name,
            load_quest_is_secondary,
            load_quest_is_active,
            save_quest_text,
            add_quest_to_queue,
            // dialog commands
            load_dialogs,
            load_speakers,
            pick_dialog_directory,
            create_new_dialog,
            create_speaker,
            update_dialog_labels,
            load_dialog_directory,
            load_dialog_labels,
            load_dialog_name,
            load_dialog_script_name,
            load_dialog_speakers,
            load_dialog_variant,
            load_variant_speaker,
            load_variant_text,
            save_dialog_variant,
            generate_dialog,
            //reserve heroes commands
            services::reserve_hero_creator::commands::load_existing_reserve_heroes,
            services::reserve_hero_creator::commands::remove_reserve_hero,
            services::reserve_hero_creator::commands::add_reserve_hero,
            //banks
            services::banks_configurator::commands::get_all_banks,
            services::banks_configurator::commands::load_bank,
            services::banks_configurator::commands::create_variant,
            services::banks_configurator::commands::load_variant,
            services::banks_configurator::commands::load_bank_variants,
            services::banks_configurator::commands::create_creature_slot,
            services::banks_configurator::commands::load_creature_slots_ids,
            services::banks_configurator::commands::load_creature_slot
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
