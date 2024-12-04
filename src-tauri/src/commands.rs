use std::path::PathBuf;

use homm5_repacker::Repacker;
use homm5_scaner::ScanExecutor;
use map_modifier::{quest, GenerateBoilerplate};
use map_modifier::quest::QuestCreationRequest;
use runtime_main::RuntimeRunner;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;

use crate::utils::Config;

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
pub async fn pick_quest_directory(
    app: AppHandle
) -> Result<(), ()> {
    app.dialog()
        .file()
        .set_can_create_directories(true)
        .pick_folder(move |f| {
            app.emit("quest_directory_picked", f.unwrap().to_string()).unwrap();
        });
    Ok(())
}

#[tauri::command]
pub async fn generate_quest(
    directory: String,
    script_name: String,
    name: String,
    desc: String,
    progresses: Vec<String>
) -> Result<(), ()> {
    let quest_generator_request = QuestCreationRequest::new(PathBuf::from(directory), script_name)
        .with_name(name)
        .with_desc(desc)
        .with_progresses(progresses)
        .secondary(false)
        .initialy_active(false);

    let quest = quest_generator_request.generate();
    map_modifier::quest::test_convert(quest);

    Ok(())
}