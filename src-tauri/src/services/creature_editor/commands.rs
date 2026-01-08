use std::{io::Write, path::PathBuf};

use homm5_scaner::prelude::{CreatureDBModel, ScanerService, UpdateCreaturePayload};
use homm5_types::creature::AdvMapCreatureShared;
use tauri::State;

use crate::{error::Error, utils::LocalAppManager};

#[tauri::command]
pub async fn load_creature(
    scaner_service: State<'_, ScanerService>,    
    id: i32,
) -> Result<Option<CreatureDBModel>, Error> {
    Ok(scaner_service.get_creature(id).await?)
}

#[tauri::command]
pub async fn update_creature_attack(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_attack(value)).await?)
}

#[tauri::command]
pub async fn update_creature_defence(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_defence(value)).await?)
}

#[tauri::command]
pub async fn update_creature_min_damage(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_min_damage(value)).await?)
}

#[tauri::command]
pub async fn update_creature_max_damage(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_max_damage(value)).await?)
}

#[tauri::command]
pub async fn update_creature_initiative(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_initiative(value)).await?)
}

#[tauri::command]
pub async fn update_creature_speed(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_speed(value)).await?)
}

#[tauri::command]
pub async fn update_creature_health(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_health(value)).await?)
}

#[tauri::command]
pub async fn update_creature_exp(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_exp(value)).await?)
}

#[tauri::command]
pub async fn update_creature_tier(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_tier(value)).await?)
}


#[tauri::command]
pub async fn update_creature_power(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_power(value)).await?)
}

#[tauri::command]
pub async fn update_creature_grow(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_health(value)).await?)
}

#[tauri::command]
pub async fn update_creature_size(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_size(value)).await?)
}

#[tauri::command]
pub async fn update_creature_range(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_range(value)).await?)
}

#[tauri::command]
pub async fn update_creature_shots(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_shots(value)).await?)
}

#[tauri::command]
pub async fn generate_creature_file(
    scaner_service: State<'_, ScanerService>,
    app_manager: State<'_, LocalAppManager>,
    id: i32
) -> Result<(), Error> {
    if let Some(creature_data) = scaner_service.get_creature(id).await? {
        let profile_locked = app_manager.current_profile_data.read().await;
        let path = PathBuf::from(format!("{}GOG_Mod/{}", &profile_locked.data_path, &creature_data.xdb_path));
        let mut file = std::fs::File::create(path)?;
        let shared: AdvMapCreatureShared = creature_data.into();
        let shared_xml = quick_xml::se::to_string(&shared)?;
        file.write_all(shared_xml.as_bytes())?;
    } 
    Ok(())
}