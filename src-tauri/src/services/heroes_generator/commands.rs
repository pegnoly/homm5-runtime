use std::path::PathBuf;
use editor_tools::prelude::{HeroAssetModel, HeroGeneratorRepo, InitGeneratableHeroPayload};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use crate::{error::Error, utils::LocalAppManager};

#[tauri::command]
pub async fn load_all_hero_assets(
    heroes_repo: State<'_, HeroGeneratorRepo>
) -> Result<Vec<HeroAssetModel>, Error> {
    Ok(heroes_repo.get_all_hero_assets().await?)
}

#[tauri::command]
pub async fn pick_hero_lua_generation_directory(
    app: AppHandle,
    app_manager: State<'_, LocalAppManager>,
) -> Result<(), ()> {
    let base_config_locked = app_manager.base_config.read().await;
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();
    let map = base_config_locked.maps.iter().find(|m| m.id == current_map_id).unwrap();

    app.dialog()
        .file()
        .set_directory(PathBuf::from(&map.data_path))
        .set_can_create_directories(true)
        .pick_folder(move |f| {
            app.emit("hero_lua_directory_picked", f.unwrap().to_string())
                .unwrap();
        });
    Ok(())
}

#[tauri::command]
pub async fn init_new_generatable_hero(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    name: String,
    path: String,
    table_name: String
) -> Result<HeroAssetModel, Error> {
    let payload = InitGeneratableHeroPayload { name, path_to_generate: path, lua_table_name: table_name};
    Ok(heroes_repo.init_new_generatable_hero(payload).await?)
}

#[tauri::command]
pub async fn load_hero_asset(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    id: i32
) -> Result<Option<HeroAssetModel>, Error> {
    Ok(heroes_repo.get_hero_asset(id).await?)
}