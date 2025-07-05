use std::{io::Write, path::PathBuf};
use homm5_scaner::prelude::ScanerService;
use tauri::State;
use crate::{error::Error, services::creature_generator::{types::{CreatableCreatureModel, CreatureGeneratorSessionConfig}, utils::process_generation}, utils::LocalAppManager};

#[tauri::command]
pub async fn save_generation_session(
    app_manager: State<'_, LocalAppManager>,
    session_name: String,
    current_id: i32,
    created_ids: Vec<i32>,
    models: Vec<CreatableCreatureModel>,
    selected_abilities: Vec<i32>
) -> Result<(), Error> {
    let config = CreatureGeneratorSessionConfig {
        current_id,
        created_ids,
        models,
        selected_abilities,
        name: session_name.clone()
    };
    let json = serde_json::to_string_pretty(&config)?;
    let base_config_locked = app_manager.base_config.read().await;
    let path = PathBuf::from(format!("{}{}.json", &base_config_locked.session_configs_path.as_ref().unwrap(), session_name));
    let mut file = std::fs::File::create(&path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[tauri::command]
pub async fn generate_creatures(
    app_manager: State<'_, LocalAppManager>,
    scaner_repo: State<'_, ScanerService>,
    models: Vec<CreatableCreatureModel>,
    selected_abilities: Vec<i32>
) -> Result<(), Error> {
    let creatures_data = scaner_repo.get_all_creature_models().await?;
    let abilities_data = scaner_repo.get_abilities().await?;
    let base_config_locked = app_manager.base_config.read().await;
    let generation_path = format!("{}GOG_Mod\\GameMechanics\\Creature\\Creatures\\Neutrals\\", &base_config_locked.data_path);

    for model in &models {
        let base_creature_data = scaner_repo.get_creature(model.base_creature.unwrap()).await?.unwrap();
        let result = process_generation(&creatures_data, &abilities_data, &base_creature_data, model, &selected_abilities).await?;
        let mut file = std::fs::File::create(format!("{}Creature_{}.xdb", &generation_path, model.id))?;
        file.write_all(result.as_bytes())?;
    } 

    Ok(())
}