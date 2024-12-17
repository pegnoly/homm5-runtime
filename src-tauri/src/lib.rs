use tokio::sync::{RwLock, Mutex};
use utils::{Config, LocalAppManager, RuntimeConfig};

pub mod commands;
pub mod utils;

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

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(cfg)
        .manage(LocalAppManager {
            runtime_config: Mutex::new(runtime_cfg),
            db_pool: RwLock::new(pool)
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::execute_scan,
            commands::run_game,
            commands::load_repackers,
            commands::repack,
            commands::generate_quest,
            commands::pick_quest_directory,
            commands::load_maps,
            commands::load_current_map,
            commands::select_map,
            commands::create_quest,
            commands::save_progress,
            commands::load_progress,
            commands::update_quest_directory,
            commands::update_quest_script_name,
            commands::update_quest_name,
            commands::update_quest_desc
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
