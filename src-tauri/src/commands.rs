use std::io::Write;
use std::path::PathBuf;

use editor_tools::prelude::{QuestGeneratorRepo, ReserveHeroCreatorRepo};
use homm5_repacker::Repacker;
use homm5_scaner::prelude::{ScanerService, Town};
use itertools::Itertools;
use map_modifier::quest::{QuestBoilerplateHelper, QuestCreationRequest, QuestProgress};
use map_modifier::{GenerateBoilerplate, MapData, ModifiersQueue};
use runtime_main::RuntimeRunner;
use tauri::State;

use crate::profiles::{ProfileConfig, ProfileType};
use crate::DataContainer;
use crate::error::Error;
use crate::utils::{LocalAppManager, MapFrontendModel, RepackerFrontendData, RuntimeData};

#[tauri::command]
pub async fn execute_scan(
    app_manager: State<'_, LocalAppManager>,
    scaner_service: State<'_, ScanerService>,
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let data_path = PathBuf::from(&profile.data_path);
    let root_folder = data_path.parent().unwrap();
    let maps_path = root_folder.join("Maps\\");
    let mods_path = root_folder.join("UserMODs\\");
    let output_path = data_path.join("MCCS_GeneratedFiles.pak");
    scaner_service
        .run(vec![data_path, maps_path, mods_path], output_path)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn run_game(
    app_manager: State<'_, LocalAppManager>,
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let mut runtime_runner = RuntimeRunner::new(PathBuf::from(format!(
        "{}{}",
        &profile.bin_path, &profile.exe_name
    )));
    runtime_runner.run();
    Ok(())
}

#[tauri::command]
pub async fn load_repackers(
    app_manager: State<'_, LocalAppManager>
) -> Result<Vec<RepackerFrontendData>, Error> {
    let profile = app_manager.current_profile_data.read().await;
    let repackers_data = profile
        .repackers
        .iter()
        .map(|(key, value)| RepackerFrontendData {
            label: key.clone(),
            update_time: value.last_update.clone(),
        })
        .collect_vec();
    Ok(repackers_data)
}

#[tauri::command]
pub async fn load_maps(
    app_manager: State<'_, LocalAppManager>
) -> Result<Vec<MapFrontendModel>, Error> {
    let profile = app_manager.current_profile_data.read().await;
    Ok(profile
        .maps
        .iter()
        .map(|m| MapFrontendModel {
            id: m.id,
            name: m.name.clone(),
        })
        .collect())
}

#[tauri::command]
pub async fn load_current_map(app_manager: State<'_, LocalAppManager>) -> Result<Option<u16>, ()> {
    Ok(app_manager.runtime_config.read().await.current_selected_map)
}

#[tauri::command]
pub async fn select_map(
    app_manager: State<'_, LocalAppManager>, 
    id: u16
) -> Result<(), ()> {
    let mut runtime_config_locked = app_manager.runtime_config.write().await;
    runtime_config_locked.current_selected_map = Some(id);
    let exe_path = std::env::current_exe().unwrap();
    let runtime_cfg_path = exe_path.parent().unwrap().join("cfg\\runtime.json");
    let new_runtime_data = serde_json::to_string_pretty(&RuntimeData {
        current_selected_map: id,
    })
    .unwrap();
    let mut file = std::fs::File::create(&runtime_cfg_path).unwrap();
    file.write_all(new_runtime_data.as_bytes()).unwrap();

    let profile: tokio::sync::RwLockReadGuard<'_, ProfileConfig> = app_manager.current_profile_data.read().await;
    let map = profile
        .maps
        .iter()
        .find(|m| m.id == runtime_config_locked.current_selected_map.unwrap())
        .unwrap();
    let current_map_data = MapData::read(map);
    let current_map_data_path = exe_path
        .parent()
        .unwrap()
        .join("cfg\\current_map_data.json");
    let current_map_data_string = serde_json::to_string_pretty(&current_map_data).unwrap();
    let mut file = std::fs::File::create(&current_map_data_path).unwrap();
    file.write_all(current_map_data_string.as_bytes()).unwrap();
    runtime_config_locked.current_map_data = current_map_data;
    Ok(())
}

#[tauri::command]
pub async fn repack(
    app_manager: State<'_, LocalAppManager>,
    repacker_label: String,
) -> Result<String, Error> {
    let mut profile = app_manager.current_profile_data.write().await;
    let base_config_locked = app_manager.base_config.read().await;
    if let Some(repacker_data) = profile.repackers.get_mut(&repacker_label) {
        let from = PathBuf::from(&repacker_data.from);
        let to = PathBuf::from(&repacker_data.to);
        let repacker = Repacker::new(from, to);
        repacker.run();
        let date = chrono::Local::now()
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
            .to_string();
        repacker_data.last_update = date.clone();
        let updated_profile_data = serde_json::to_string_pretty(&*profile).unwrap();
        let exe_path = std::env::current_exe()?;
        let profile_cfg_path = exe_path.parent().unwrap().join(format!("cfg\\{}\\main.json", base_config_locked.current_profile));
        let mut file = std::fs::File::create(&profile_cfg_path)?;
        file.write_all(updated_profile_data.as_bytes())?;
        Ok(date)
    } else {
        Err(Error::UndefinedData("Repacker to update".to_string()))
    }
}

#[tauri::command]
pub async fn apply_modifications(
    app_manager: State<'_, LocalAppManager>,
    quests_repo: State<'_, QuestGeneratorRepo>,
    data_containter: State<'_, DataContainer>,
    reserve_heroes_repo: State<'_, ReserveHeroCreatorRepo>,
) -> Result<(), super::error::Error> {
    let mut runtime_config_locked = app_manager.runtime_config.write().await;
    let profile = app_manager.current_profile_data.read().await;
    let current_map_id = runtime_config_locked.current_selected_map.unwrap();
    let map = profile
        .maps
        .iter()
        .find(|m| m.id == current_map_id)
        .unwrap();
    let mod_path = &profile.mod_path;

    let mut modifiers_queue = ModifiersQueue::new(
        &data_containter.banks,
        &data_containter.buildings,
        &data_containter.artifacts,
    );

    // get all ids of quest for current map

    // get all quests data for these ids and convert db models to quests

    let this_mission_quests = quests_repo.load_quests(current_map_id as i32).await?;
    for model in &this_mission_quests {
        let progresses = quests_repo.load_progresses(model.id).await?;
        let request = QuestCreationRequest::new(
            PathBuf::from(model.directory.clone()),
            model.script_name.clone(),
        )
        .with_name(model.name.clone())
        .with_desc(model.desc.clone())
        .with_progresses(
            progresses
                .into_iter()
                .map(|p| QuestProgress {
                    number: p.number as u32,
                    text: p.text,
                    concatenate: p.concatenate,
                })
                .collect(),
        )
        .with_mission_data(map.campaign, map.mission)
        .secondary(model.is_secondary)
        .initialy_active(model.is_active);

        let quest = request.generate(Some(&QuestBoilerplateHelper {
            mod_path: mod_path.clone(),
            map_data_path: map.data_path.clone(),
            texts_path: profile.texts_path.clone(),
        }))?;
        if model.is_secondary {
            modifiers_queue.secondary_quests.push(quest);
        } else {
            modifiers_queue.primary_quests.push(quest);
        }
    }

    println!("Primary quests: {:?}", &modifiers_queue.primary_quests);
    println!("Secondary quests: {:?}", &modifiers_queue.secondary_quests);

    modifiers_queue
        .apply_changes_to_map(
            map,
            &mut runtime_config_locked.current_map_data,
            &reserve_heroes_repo,
        )
        .await;

    Ok(())
}

#[tauri::command]
pub async fn create_hero(
    app_manager: State<'_, LocalAppManager>,
    profile_data: State<'_, ProfileConfig>,
    hero_name: String,
    hero_script_name: String,
    town: Town,
) -> Result<(), Error> {
    let global_config_locked = app_manager.base_config.read().await;
    editor_tools::prelude::process_files(
        &PathBuf::from(global_config_locked.generic_hero_xdb.as_ref().unwrap()),
        &PathBuf::from(global_config_locked.generic_icon_128.as_ref().unwrap()),
        &PathBuf::from(global_config_locked.generic_icon_dds.as_ref().unwrap()),
        format!("{}GOG_Mod\\", &profile_data.data_path),
        town,
        hero_script_name,
        hero_name,
    )?;
    Ok(())
}

#[tauri::command]
pub async fn switch_profile(
    app_manager: State<'_, LocalAppManager>,
    new_profile: ProfileType
) -> Result<(), Error> {
    let mut base_config_locked = app_manager.base_config.write().await;
    let mut profile = app_manager.current_profile_data.write().await; 
    let exe_path = std::env::current_exe()?;
    let new_profile_path = exe_path.parent().unwrap().join(format!("cfg\\{new_profile}\\profile.json"));
    let new_profile_data = serde_json::from_str::<ProfileConfig>(&std::fs::read_to_string(new_profile_path)?)?;
    *profile = new_profile_data;
    base_config_locked.current_profile = new_profile;
    Ok(())
}