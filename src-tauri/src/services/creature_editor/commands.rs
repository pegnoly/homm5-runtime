use std::{io::Write, path::PathBuf};

use filetime::{FileTime, set_file_mtime};
use homm5_scaner::prelude::{CreatureDBModel, MagicElementModel, Mastery, ResourcesModel, ScanerService, SpellWithMasteryModel, Town, UpdateCreaturePayload};
use homm5_types::creature::AdvMapCreatureShared;
use quick_xml::{Writer, events::{BytesDecl, Event}};
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
pub async fn update_creature_base_creature(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: String
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_base_creature(value)).await?)
}

#[tauri::command]
pub async fn update_creature_pair_creature(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: String
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_pair_creature(value)).await?)
}

#[tauri::command]
pub async fn update_creature_upgrades(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: Vec<String>
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_upgrades(value)).await?)
}

#[tauri::command]
pub async fn update_creature_abilities(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: Vec<String>
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_abilities(value)).await?)
}

#[tauri::command]
pub async fn update_creature_magic_element(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: MagicElementModel
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_magic_element(value)).await?)
}

#[tauri::command]
pub async fn update_creature_cost(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: ResourcesModel
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_cost(value)).await?)
}

#[tauri::command]
pub async fn update_creature_is_generatable(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: bool
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_generatable(value)).await?)
}

#[tauri::command]
pub async fn update_creature_is_upgrade(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: bool
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_upgrade(value)).await?)
}

#[tauri::command]
pub async fn update_creature_is_flying(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: bool
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_flying(value)).await?)
}

#[tauri::command]
pub async fn remove_creature_spell(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: String
) -> Result<(), Error> {
    Ok(scaner_service.remove_creature_spell(id, value).await?)
}

#[tauri::command]
pub async fn add_creature_spell(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    spell: String,
    mastery: Mastery
) -> Result<(), Error> {
    Ok(scaner_service.add_creature_spell(id, spell, mastery).await?)
}

#[tauri::command]
pub async fn update_creature_spell(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    curr_spell: String,
    new_spell: SpellWithMasteryModel
) -> Result<(), Error> {
    Ok(scaner_service.update_creature_spell(id, curr_spell, new_spell).await?)
}

#[tauri::command]
pub async fn update_creature_town(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: Town,
) -> Result<(), Error> {
    Ok(scaner_service.update_creature(UpdateCreaturePayload::new(id).with_town(value)).await?)
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
        let mut file = std::fs::File::create(&path)?;
        let time = FileTime::from_unix_time(4573920000i64, 0);
        let mut output: Vec<u8> = Vec::new();
        let mut writer = Writer::new_with_indent(&mut output, b' ', 4);
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
        let mut shared: AdvMapCreatureShared = creature_data.into();
        if shared.KnownSpells.is_some() && shared.KnownSpells.as_ref().unwrap().spells.is_none() {
            shared.KnownSpells = None;
        }
        if shared.PatternAttack.is_some() && shared.PatternAttack.as_ref().unwrap().PatternAttack.is_none() {
            shared.PatternAttack = None;
        }
        if shared.flybySequence.is_some() && shared.flybySequence.as_ref().unwrap().Item.is_none() {
            shared.flybySequence = None;
        }
        writer.write_serializable("Creature", &shared)?;
        file.write_all(&output)?;
        set_file_mtime(path, time).unwrap();
    } 
    Ok(())
}