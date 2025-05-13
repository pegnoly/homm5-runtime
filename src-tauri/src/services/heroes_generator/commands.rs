use std::path::PathBuf;
use editor_tools::prelude::{AddOptionalArtifactPayload, AddRequiredArtifactPayload, AssetGenerationType, DifficultyType, HeroAssetArtifactsModel, HeroAssetModel, HeroGeneratorRepo, InitAssetArtifactsDataPayload, InitGeneratableHeroPayload, RemoveOptionalArtifactPayload, RemoveRequiredArtifactPayload, UpdateArtifactsGenerationPowerPayload};
use homm5_scaner::prelude::{ArtifactDBModel, ArtifactSlotType, ScanerService};
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

#[tauri::command]
pub async fn load_artifact_models(
    scaner_repo: State<'_, ScanerService>
) -> Result<Vec<ArtifactDBModel>, Error> {
    Ok(scaner_repo.get_artifact_models().await?)
}

#[tauri::command]
pub async fn try_load_artifacts_data_for_asset(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32
) -> Result<Option<HeroAssetArtifactsModel>, Error> {
    Ok(heroes_repo.get_artifacts_model(asset_id).await?)
}

#[tauri::command]
pub async fn create_artifacts_data_for_asset(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    generation_type: AssetGenerationType
) -> Result<HeroAssetArtifactsModel, Error> {
    let payload = InitAssetArtifactsDataPayload { asset_id, generation_type };
    Ok(heroes_repo.add_artifacts_model(payload).await?)
}

#[tauri::command]
pub async fn update_artifacts_base_cost(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    heroes_repo.update_artifacts_base_generation_power(UpdateArtifactsGenerationPowerPayload {id: container_id, difficulty, value: actual_value}).await?;
    Ok(actual_value)
}

#[tauri::command]
pub async fn update_artifacts_cost_grow(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    heroes_repo.update_artifacts_grow_power(UpdateArtifactsGenerationPowerPayload {id: container_id, difficulty, value: actual_value}).await?;
    Ok(actual_value)
}

#[tauri::command]
pub async fn add_required_artifact(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    artifact_id: i32
) -> Result<(), Error> {
    Ok(heroes_repo.add_required_artifact_id(AddRequiredArtifactPayload { asset_id, artifact_id}).await?)
}

#[tauri::command]
pub async fn remove_required_artifact(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    artifact_id: i32
) -> Result<(), Error> {
    Ok(heroes_repo.remove_required_artifact_id(RemoveRequiredArtifactPayload { asset_id, artifact_id}).await?)
}

#[tauri::command]
pub async fn add_optional_artifact(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    slot: ArtifactSlotType,
    artifact_id: i32
) -> Result<(), Error> {
    Ok(heroes_repo.add_optional_artifact_id(AddOptionalArtifactPayload { asset_id, slot, artifact_id}).await?)
}

#[tauri::command]
pub async fn remove_optional_artifact(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    slot: ArtifactSlotType,
    artifact_id: i32
) -> Result<(), Error> {
    Ok(heroes_repo.remove_optional_artifact_id(RemoveOptionalArtifactPayload { asset_id, slot, artifact_id}).await?)
}