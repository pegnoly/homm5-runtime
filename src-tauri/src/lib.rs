use services::QuestService;
use tokio::sync::Mutex;
use utils::{Config, LocalAppManager, RuntimeConfig};

mod services;
mod commands;
mod utils;
mod source;
mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let exe_path = std::env::current_exe().unwrap();
    let cfg_path = exe_path.parent().unwrap().join("cfg\\");

    let cfg_string = std::fs::read_to_string(&cfg_path.join("main.json")).unwrap();
    let cfg: Config = serde_json::from_str(&cfg_string).unwrap();

    let runtime_cfg_string = std::fs::read_to_string(&cfg_path.join("runtime.json")).unwrap();
    let runtime_cfg: RuntimeConfig = serde_json::from_str(&runtime_cfg_string).unwrap();

    let db_path = cfg_path.join("runtime_database.db");
    if db_path.exists() == false {
        std::fs::File::create(&db_path).unwrap();
    }

    let pool = sqlx::SqlitePool::connect(&db_path.to_str().unwrap()).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let quest_service = QuestService::new(pool);

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(cfg)
        .manage(LocalAppManager {
            runtime_config: Mutex::new(runtime_cfg),
        })
        .manage(quest_service)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::execute_scan,
            commands::run_game,
            commands::load_repackers,
            commands::repack,
            commands::add_quest_to_queue,
            commands::pick_quest_directory,
            commands::load_maps,
            commands::load_current_map,
            commands::select_map,
            commands::create_quest,
            commands::save_progress,
            commands::load_progress,
            commands::update_progress_concatenation,
            commands::update_quest_directory,
            commands::update_quest_script_name,
            commands::update_quest_name,
            commands::update_quest_desc,
            commands::apply_modifications,
            commands::update_is_secondary,
            commands::update_is_active,
            commands::save_quest_text,

            commands::collect_quests_for_selection,
            commands::load_quest_name,
            commands::load_quest_desc,
            commands::load_quest_directory,
            commands::load_quest_script_name,
            commands::load_quest_is_secondary,
            commands::load_quest_is_active
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
