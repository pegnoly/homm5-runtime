use utils::Config;

pub mod commands;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let exe_path = std::env::current_exe().unwrap();
    let cfg_path = exe_path.parent().unwrap().join("cfg\\main.json");

    let cfg_string = std::fs::read_to_string(&cfg_path).unwrap();
    let cfg: Config = serde_json::from_str(&cfg_string).unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(cfg)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::execute_scan,
            commands::run_game,
            commands::load_repackers,
            commands::repack,
            commands::generate_quest,
            commands::pick_quest_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
