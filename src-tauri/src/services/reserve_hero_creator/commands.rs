use std::io::Write;

use homm5_scaner::entity::hero;
use map_modifier::FileRef;
use tauri::State;
use crate::utils::LocalAppManager;
use homm5_types::{common::{Action, Pos, Trigger}, hero::{AdvMapHero, ArtifactIds, Editable, FavoriteEnemies, IsUntransferable, Perks, Skills, SpellIds, Textures}, player::PlayerID, town::ArmySlots};

#[tauri::command]
pub async fn load_existing_reserve_heroes(
    app_manager: State<'_, LocalAppManager>,
    player: i32
) -> Result<Vec<String>, ()> {
    let config_locked = app_manager.runtime_config.lock().await;
    let current_map_data = &config_locked.current_map_data;
    if let Some(heroes) = current_map_data.reserve_heroes.get(&player) {
        Ok(heroes.into_iter().map(|h| {
            h.shared.href.as_ref().unwrap().clone()
        }).collect::<Vec<String>>())
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
pub async fn remove_reserve_hero(
    app_manager: State<'_, LocalAppManager>,
    hero: String,
    player: i32
) -> Result<(), ()> {
    let mut config_locked = app_manager.runtime_config.lock().await;
    let current_map_data = &mut config_locked.current_map_data;
    let updated_heroes: Vec<AdvMapHero> = current_map_data.reserve_heroes.get(&player).unwrap().iter()
        .filter(|h| {
            *h.shared.href.as_ref().unwrap() != hero
        })
        .map(|h| {
            h.clone()
        })
        .collect();

    current_map_data.reserve_heroes.insert(player, updated_heroes);

    let exe_path = std::env::current_exe().unwrap();
    let current_map_data_path = exe_path.parent().unwrap().join("cfg\\current_map_data.json");
    let current_map_data_string = serde_json::to_string_pretty(&current_map_data).unwrap();
    let mut file = std::fs::File::create(&current_map_data_path).unwrap();
    file.write_all(&current_map_data_string.as_bytes()).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn add_reserve_hero(
    app_manager: State<'_, LocalAppManager>,
    hero: String,
    player: i32
) -> Result<(), ()> {
    let new_hero = AdvMapHero {
        pos: Pos {x: 0.0, y: 0.0, z: 0.0},
        rot: 0.0,
        floor: 0,
        name: None,
        combat_script: None,
        point_lights: None,
        shared: FileRef {href: Some(hero)},
        player_id: PlayerID::from_repr(player).unwrap_or(PlayerID::Player1),
        experience: 0,
        army_slots: Some(ArmySlots {army_slots: None}),
        artifact_ids: Some(ArtifactIds { items: None}),
        is_untransferable: Some(IsUntransferable {items: None}),
        editable: Editable {
            NameFileRef: None,
            BiographyFileRef: None,
            Offence: 0,
            Defence: 0,
            Spellpower: 0,
            Knowledge: 0,
            skills: Some(Skills {items: None}),
            perkIDs: Some(Perks {items: None}),
            spellIDs: Some(SpellIds {items: None}),
            Ballista: false,
            FirstAidTent: false,
            AmmoCart: false,
            FavoriteEnemies: Some(FavoriteEnemies {items: None}),
            TalismanLevel: Some(0)
        },
        override_mask: 0,
        primary_skill_mastery: "MASTERY_BASIC".to_string(),
        loss_trigger: Trigger { action: Action { function_name: None }},
        allow_quick_combat: true,
        textures: Textures {
            icon128: None,
            icon64: None,
            rounded_face: None,
            left_face: None,
            right_face: None
        },
        preset_price: 0,
        banned_races: None
    };

    let mut config_locked = app_manager.runtime_config.lock().await;
    let current_map_data = &mut config_locked.current_map_data;
    if let Some(heroes) = current_map_data.reserve_heroes.get_mut(&player) {
        heroes.push(new_hero);
    }
    
    let exe_path = std::env::current_exe().unwrap();
    let current_map_data_path = exe_path.parent().unwrap().join("cfg\\current_map_data.json");
    let current_map_data_string = serde_json::to_string_pretty(&current_map_data).unwrap();
    let mut file = std::fs::File::create(&current_map_data_path).unwrap();
    file.write_all(&current_map_data_string.as_bytes()).unwrap();

    Ok(())
}