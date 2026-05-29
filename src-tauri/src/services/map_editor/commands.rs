use std::io::Write;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use crate::utils::LocalAppManager;

#[tauri::command]
pub async fn pick_map_xdb_directory(
    app: AppHandle,
    app_manager: State<'_, LocalAppManager>,
) -> Result<(), crate::Error> {
    let profile = app_manager.current_profile_data.read().await;
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();
    let map = profile
        .maps
        .iter()
        .find(|m| m.id == current_map_id)
        .unwrap();

    let map_path = PathBuf::from(&map.data_path);

    let actual_path = map_path.ancestors()
        .find(|p| p.ends_with("UserCampaigns"))
        .unwrap();
    let actual_path = actual_path.parent().unwrap().join("Maps");

    app.dialog()
        .file()
        .set_directory(PathBuf::from(&actual_path))
        .set_can_create_directories(false)
        .pick_folder(move |f| {
            app.emit("map_directory_picked", f.unwrap().to_string())
                .unwrap();
        });
    Ok(())
}

#[tauri::command]
pub async fn move_map_to_dir(
    app_manager: State<'_, LocalAppManager>,
    path: String,
    size: i32
) -> Result<(), crate::Error> {
    let global_config = app_manager.base_config.read().await;
    let map_path = global_config.generic_map_xdb.clone().unwrap();
    let mut map_data = std::fs::read_to_string(&map_path)?;
    map_data = map_data.replace("<TileX>72</TileX>", &format!("<TileX>{size}</TileX>"))
        .replace("<TileY>72</TileY>", &format!("<TileY>{size}</TileY>"));
    let dest_path = PathBuf::from(&path).join("map.xdb");
    let mut file = std::fs::File::create(&dest_path)?;
    file.write_all(map_data.as_bytes())?;
    Ok(())
}