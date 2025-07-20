use crate::error::Error;
use editor_tools::prelude::{BaseSkill, InitReserveHeroPayload, ReserveHeroCreatorRepo, ReserveHeroModel};
use homm5_scaner::prelude::{HeroDBModel, MagicSchoolType, ScanerService, SkillDBModel, SpellDBModel, Town};
use tauri::State;

#[tauri::command]
pub async fn load_heroes_data(
    scaner_repo: State<'_, ScanerService>,
    town: Town
) -> Result<Vec<HeroDBModel>, Error> {
    Ok(scaner_repo.get_heroes_models(town).await?)
}

#[tauri::command]
pub async fn load_base_skills(
    scaner_repo: State<'_, ScanerService>
) -> Result<Vec<SkillDBModel>, Error> {
    Ok(scaner_repo.get_base_skills().await?)
}

#[tauri::command]
pub async fn load_heroes(
    reserve_heroes_repo: State<'_, ReserveHeroCreatorRepo>,
    map_id: i32,
    player: i32 
) -> Result<Vec<ReserveHeroModel>, Error> {
    let heroes = reserve_heroes_repo.load_heroes(map_id, player).await?;
    Ok(heroes)
}

#[tauri::command]
pub async fn load_existing_reserved_hero(
    reserve_heroes_repo: State<'_, ReserveHeroCreatorRepo>,
    id: i32
) -> Result<ReserveHeroModel, Error> {
    Ok(reserve_heroes_repo.load_existing_hero(id).await?)
}

#[tauri::command]
pub async fn init_new_hero(
    reserve_heroes_repo: State<'_, ReserveHeroCreatorRepo>,
    map_id: i32,
    player_id: i32,
    name: String,
    xdb: String
) -> Result<ReserveHeroModel, Error> {
    Ok(reserve_heroes_repo.init_hero(InitReserveHeroPayload { map_id, player_id, name, xdb }).await?)
}

#[tauri::command]
pub async fn add_skill(
    reserve_heroes_repo: State<'_, ReserveHeroCreatorRepo>,
    id: i32,
    slot: i32
) -> Result<BaseSkill, Error> {
    Ok(reserve_heroes_repo.add_skill(id, slot).await?)
}

#[tauri::command]
pub async fn update_skill(
    reserve_heroes_repo: State<'_, ReserveHeroCreatorRepo>,
    id: i32,
    slot: i32,
    skill: BaseSkill
) -> Result<(), Error> {
    Ok(reserve_heroes_repo.update_skill(id, slot, skill).await?)
}

#[tauri::command]
pub async fn load_perks(
    scaner_repo: State<'_, ScanerService>,
    skill: String
) -> Result<Vec<SkillDBModel>, Error> {
    Ok(scaner_repo.get_perks_for_skill(skill).await?)
}

#[tauri::command]
pub async fn load_spells(
    scaner_repo: State<'_, ScanerService>,
    school: MagicSchoolType
) -> Result<Vec<SpellDBModel>, Error> {
    Ok(scaner_repo.get_spells_for_school(school).await?)
}

#[tauri::command]
pub async fn add_spell(
    reserve_heroes_repo: State<'_, ReserveHeroCreatorRepo>,
    id: i32,
    spell: String
) -> Result<(), Error> {
    Ok(reserve_heroes_repo.add_spell(id, spell).await?)
}

#[tauri::command]
pub async fn remove_spell(
    reserve_heroes_repo: State<'_, ReserveHeroCreatorRepo>,
    id: i32,
    spell: String
) -> Result<(), Error> {
    Ok(reserve_heroes_repo.remove_spell(id, spell).await?)
}