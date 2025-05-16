use crate::{error::Error, utils::LocalAppManager};
use editor_tools::prelude::{
    AddGenerationStatElementPayload, AddOptionalArtifactPayload, AddRequiredArtifactPayload,
    AddStackPayload, ArmyGenerationRuleParam, ArmyGenerationStatParam, ArmyGenerationStatRule,
    ArmySlotStackCountGenerationMode, ArmySlotStackUnitGenerationMode, ArmyStatGenerationModel,
    AssetGenerationType, DifficultyType, HeroAssetArmySlotModel, HeroAssetArtifactsModel,
    HeroAssetModel, HeroGeneratorRepo, InitAssetArtifactsDataPayload, InitGeneratableHeroPayload,
    RemoveOptionalArtifactPayload, RemoveRequiredArtifactPayload,
    UpdateDifficultyBasedPowerPayload, UpdateGenerationRulesPayload,
    UpdateGenerationStatElementPayload, UpdateGenerationStatParamsPayload,
    UpdateStackConcreteCreaturePayload, UpdateStackCreatureTierPayload,
    UpdateStackCreatureTownPayload,
};
use homm5_scaner::prelude::{
    ArtifactDBModel, ArtifactSlotType, CreatureDBModel, ScanerService, Town,
};
use std::{io::Write, path::PathBuf};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn load_all_hero_assets(
    heroes_repo: State<'_, HeroGeneratorRepo>,
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
    let map = base_config_locked
        .maps
        .iter()
        .find(|m| m.id == current_map_id)
        .unwrap();

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
    table_name: String,
) -> Result<HeroAssetModel, Error> {
    let payload = InitGeneratableHeroPayload {
        name,
        path_to_generate: path,
        lua_table_name: table_name,
    };
    Ok(heroes_repo.init_new_generatable_hero(payload).await?)
}

#[tauri::command]
pub async fn load_hero_asset(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    id: i32,
) -> Result<Option<HeroAssetModel>, Error> {
    Ok(heroes_repo.get_hero_asset(id).await?)
}

#[tauri::command]
pub async fn load_artifact_models(
    scaner_repo: State<'_, ScanerService>,
) -> Result<Vec<ArtifactDBModel>, Error> {
    Ok(scaner_repo.get_artifact_models().await?)
}

#[tauri::command]
pub async fn load_creature_models(
    scaner_repo: State<'_, ScanerService>,
) -> Result<Vec<CreatureDBModel>, Error> {
    Ok(scaner_repo.get_creature_models().await?)
}

#[tauri::command]
pub async fn try_load_artifacts_data_for_asset(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
) -> Result<Option<HeroAssetArtifactsModel>, Error> {
    Ok(heroes_repo.get_artifacts_model(asset_id).await?)
}

#[tauri::command]
pub async fn create_artifacts_data_for_asset(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    generation_type: AssetGenerationType,
) -> Result<HeroAssetArtifactsModel, Error> {
    let payload = InitAssetArtifactsDataPayload {
        asset_id,
        generation_type,
    };
    Ok(heroes_repo.add_artifacts_model(payload).await?)
}

#[tauri::command]
pub async fn update_artifacts_base_cost(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String,
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    heroes_repo
        .update_artifacts_base_generation_power(UpdateDifficultyBasedPowerPayload {
            id: container_id,
            difficulty,
            value: actual_value,
        })
        .await?;
    Ok(actual_value)
}

#[tauri::command]
pub async fn update_artifacts_cost_grow(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String,
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    heroes_repo
        .update_artifacts_grow_power(UpdateDifficultyBasedPowerPayload {
            id: container_id,
            difficulty,
            value: actual_value,
        })
        .await?;
    Ok(actual_value)
}

#[tauri::command]
pub async fn add_required_artifact(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    artifact_id: i32,
) -> Result<(), Error> {
    Ok(heroes_repo
        .add_required_artifact_id(AddRequiredArtifactPayload {
            asset_id,
            artifact_id,
        })
        .await?)
}

#[tauri::command]
pub async fn remove_required_artifact(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    artifact_id: i32,
) -> Result<(), Error> {
    Ok(heroes_repo
        .remove_required_artifact_id(RemoveRequiredArtifactPayload {
            asset_id,
            artifact_id,
        })
        .await?)
}

#[tauri::command]
pub async fn add_optional_artifact(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    slot: ArtifactSlotType,
    artifact_id: i32,
) -> Result<(), Error> {
    Ok(heroes_repo
        .add_optional_artifact_id(AddOptionalArtifactPayload {
            asset_id,
            slot,
            artifact_id,
        })
        .await?)
}

#[tauri::command]
pub async fn remove_optional_artifact(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    slot: ArtifactSlotType,
    artifact_id: i32,
) -> Result<(), Error> {
    Ok(heroes_repo
        .remove_optional_artifact_id(RemoveOptionalArtifactPayload {
            asset_id,
            slot,
            artifact_id,
        })
        .await?)
}

#[tauri::command]
pub async fn load_stacks_ids(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
) -> Result<Vec<i32>, Error> {
    Ok(heroes_repo.get_stacks_ids(asset_id).await?)
}

#[tauri::command]
pub async fn create_stack(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
    type_generation_mode: ArmySlotStackUnitGenerationMode,
    count_generation_mode: ArmySlotStackCountGenerationMode,
    generation_type: Option<AssetGenerationType>,
) -> Result<i32, Error> {
    let mut payload = AddStackPayload::new(
        asset_id,
        type_generation_mode,
        count_generation_mode.clone(),
    );
    if count_generation_mode == ArmySlotStackCountGenerationMode::PowerBased {
        payload = payload.with_power_based_generation_type(
            generation_type.unwrap_or(AssetGenerationType::Static),
        )
    }
    Ok(heroes_repo.add_stack(payload).await?)
}

#[tauri::command]
pub async fn load_stack(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    stack_id: i32,
) -> Result<Option<HeroAssetArmySlotModel>, Error> {
    Ok(heroes_repo.get_stack(stack_id).await?)
}

#[tauri::command]
pub async fn update_stack_base_powers(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String,
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    heroes_repo
        .update_stack_base_power(UpdateDifficultyBasedPowerPayload {
            id: container_id,
            difficulty,
            value: actual_value,
        })
        .await?;
    Ok(actual_value)
}

#[tauri::command]
pub async fn update_stack_powers_grow(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String,
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    heroes_repo
        .update_stack_base_power(UpdateDifficultyBasedPowerPayload {
            id: container_id,
            difficulty,
            value: actual_value,
        })
        .await?;
    Ok(actual_value)
}

#[tauri::command]
pub async fn update_stack_concrete_count(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String,
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    heroes_repo
        .update_stack_creature_count(UpdateDifficultyBasedPowerPayload {
            id: container_id,
            difficulty,
            value: actual_value,
        })
        .await?;
    Ok(actual_value)
}

#[tauri::command]
pub async fn update_stack_concrete_creature(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    stack_id: i32,
    creature: i32,
) -> Result<(), Error> {
    Ok(heroes_repo
        .update_stack_concrete_creature(UpdateStackConcreteCreaturePayload { stack_id, creature })
        .await?)
}

#[tauri::command]
pub async fn update_stack_creature_town(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    stack_id: i32,
    town: Town,
) -> Result<(), Error> {
    Ok(heroes_repo
        .update_stack_creature_town(UpdateStackCreatureTownPayload { stack_id, town })
        .await?)
}

#[tauri::command]
pub async fn update_stack_creature_tier(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    stack_id: i32,
    tier: i32,
) -> Result<(), Error> {
    Ok(heroes_repo
        .update_stack_creature_tier(UpdateStackCreatureTierPayload { stack_id, tier })
        .await?)
}

#[tauri::command]
pub async fn add_stack_generation_rule(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    stack_id: i32,
    rule: ArmyGenerationRuleParam,
) -> Result<(), Error> {
    Ok(heroes_repo
        .add_generation_rule(UpdateGenerationRulesPayload { stack_id, rule })
        .await?)
}

#[tauri::command]
pub async fn remove_stack_generation_rule(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    stack_id: i32,
    rule: ArmyGenerationRuleParam,
) -> Result<(), Error> {
    Ok(heroes_repo
        .remove_generation_rule(UpdateGenerationRulesPayload { stack_id, rule })
        .await?)
}

#[tauri::command]
pub async fn load_stats_generation_elements(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    stack_id: i32,
) -> Result<Vec<ArmyStatGenerationModel>, Error> {
    Ok(heroes_repo.get_stat_generation_elements(stack_id).await?)
}

#[tauri::command]
pub async fn add_stat_generation_element(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    stack_id: i32,
    rule: ArmyGenerationStatRule,
) -> Result<ArmyStatGenerationModel, Error> {
    Ok(heroes_repo
        .add_stat_generation_element(AddGenerationStatElementPayload { stack_id, rule })
        .await?)
}

#[tauri::command]
pub async fn remove_stat_generation_element(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    element_id: i32,
) -> Result<(), Error> {
    Ok(heroes_repo
        .delete_stat_generation_element(element_id)
        .await?)
}

#[tauri::command]
pub async fn update_stat_generation_element_priority(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    element_id: i32,
    priority: i32,
) -> Result<(), Error> {
    Ok(heroes_repo
        .update_stat_generation_element(
            UpdateGenerationStatElementPayload::new(element_id).with_priority(priority),
        )
        .await?)
}

#[tauri::command]
pub async fn update_stat_generation_element_rule(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    element_id: i32,
    rule: ArmyGenerationStatRule,
) -> Result<(), Error> {
    Ok(heroes_repo
        .update_stat_generation_element(
            UpdateGenerationStatElementPayload::new(element_id).with_rule(rule),
        )
        .await?)
}

#[tauri::command]
pub async fn update_stat_generation_params(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    element_id: i32,
    params: Vec<ArmyGenerationStatParam>,
) -> Result<(), Error> {
    Ok(heroes_repo
        .update_stat_generation_params(UpdateGenerationStatParamsPayload { element_id, params })
        .await?)
}

#[tauri::command]
pub async fn generate_current_hero_script(
    heroes_repo: State<'_, HeroGeneratorRepo>,
    asset_id: i32,
) -> Result<(), Error> {
    if let Some(main_asset) = heroes_repo.get_hero_asset(asset_id).await? {
        let mut output_file =
            std::fs::File::create(format!("{}\\script.lua", &main_asset.path_to_generate))?;
        let mut script = format!("{} = {{\n", &main_asset.table_name);
        // stacks script
        let stacks_assets = heroes_repo.get_stacks(main_asset.id).await?;
        let mut stack_gen_type_script = String::from("\tstack_count_generation_logic = {\n");
        let mut base_army_count_data_script = String::from("\tarmy_base_count_data = {\n");
        let mut army_count_grow_script = String::from("\tarmy_counts_grow = {\n");
        let mut army_generation_rules_script = String::from("\tarmy_getters = {\n");

        let mut stack_count = 0;
        for asset in stacks_assets {
            stack_count += 1;

            stack_gen_type_script += &format!("\t\t[{}] = {},\n", stack_count, &asset.count_generation_mode);
            base_army_count_data_script += &format!("\t\t[{}] = {{\n", stack_count);

            if asset.count_generation_mode == ArmySlotStackCountGenerationMode::PowerBased {
                for (difficulty, power) in asset.base_powers.data {
                    base_army_count_data_script += &format!("\t\t\t[{}] = {},\n", &difficulty, power);
                }
                base_army_count_data_script += "\t\t},\n";

                if asset.power_based_generation_type == AssetGenerationType::Dynamic {
                    army_count_grow_script += &format!("\t\t[{}] = {{\n", stack_count);
                    for (difficulty, power) in asset.powers_grow.data {
                        army_count_grow_script += &format!("\t\t\t[{}] = {},\n", &difficulty, power);
                    }
                    army_count_grow_script += "\t\t},\n";
                }
            } else {
                for (difficulty, power) in asset.concrete_count.data {
                    base_army_count_data_script += &format!("\t\t\t[{}] = {},\n", &difficulty, power);
                }
                base_army_count_data_script += "\t\t},\n";
            }

            // construct generation rule filter
            if asset.type_generation_mode == ArmySlotStackUnitGenerationMode::ConcreteUnit {
                army_generation_rules_script += &format!(
                    "\t\t[{}] = function ()\n\t\t\treturn {}\n\t\tend,\n",
                    stack_count, asset.concrete_creature
                );   
            } else {

                army_generation_rules_script += &format!("\t\t[{}] = function ()\n", stack_count);

                let mut generation_rules_script = String::from("local result = ");
                for rule in &asset.generation_rule.params {
                    match rule {
                        ArmyGenerationRuleParam::Generatable => {
                            generation_rules_script += "Creature.Params.IsGeneratable(creature) and "
                        }
                        ArmyGenerationRuleParam::Caster => {
                            generation_rules_script += "Creature.Type.IsCaster(creature) and "
                        }
                        ArmyGenerationRuleParam::Shooter => {
                            generation_rules_script += "Creature.Type.IsShooter(creature) and"
                        }
                        _ => {}
                    }
                }
                generation_rules_script = generation_rules_script
                    .trim_end()
                    .trim_end_matches("and")
                    .trim_end()
                    .to_string();
                // construct getter function
                let mut inner_getter_function = format!(
                    "\t\t\tlocal tiers = TIER_TABLES[{}][{}]\n",
                    asset.town, asset.tier
                );
                let filter_function = format!(
                    "\t\t\tlocal filtered_tiers = list_iterator.Filter(\n\t\t\t\ttiers,\n\t\t\t\tfunction(creature)\n\t\t\t\t\t{}\n\t\t\t\t\treturn result\n\t\t\t\tend)", &generation_rules_script);
                
                let stats_elements = heroes_repo.get_stat_generation_elements(asset.id).await?;
                //println!("Stats elements: {:#?}", &stats_elements);
                if stats_elements.len() == 0 {
                    if asset.generation_rule.params.len() > 0 {
                        inner_getter_function += &format!(
                            "{}\n\t\t\tlocal id = Random.FromTable(filtered_tiers)\n\t\t\treturn id",
                            &filter_function
                        );
                    } else {
                        inner_getter_function +=
                            "\t\t\tlocal id = Random.FromTable(tiers)\n\t\t\treturn id";
                    }
                    army_generation_rules_script += &format!("{}\n\t\tend,\n", inner_getter_function);   
                } else {
                    let stat_element = &stats_elements[0];
                    let mut sort_function = format!(
                        "{}\n\t\t\tlocal id = list_iterator.{}(\n\t\t\t\t{},\n\t\t\t\tfunction(creature)\n\t\t\t\t\tlocal result = ", 
                            if asset.generation_rule.params.len() > 0 { 
                                format!("\t\t\tlocal tiers = TIER_TABLES[{}][{}]\n{}", asset.town, asset.tier, filter_function) 
                            } else { 
                                format!("\t\t\tlocal tiers = TIER_TABLES[{}][{}]", asset.town, asset.tier)
                            },                         
                            if stat_element.rule == ArmyGenerationStatRule::MaxBy { "MaxBy" } else { "MinBy" },
                            if asset.generation_rule.params.len() > 0 { "filtered_tiers" } else { "tiers" }
                        );
                    for param in &stat_element.stats.values {
                        match param {
                            ArmyGenerationStatParam::Attack => {
                                sort_function += "Creature.Params.Attack(creature) + ";
                            },
                            ArmyGenerationStatParam::Defence => {
                                sort_function += "Creature.Params.Defence(creature) + ";
                            },
                            ArmyGenerationStatParam::Initiative => {
                                sort_function += "Creature.Params.Initiative(creature) + ";
                            },
                            ArmyGenerationStatParam::Speed => {
                                sort_function += "Creature.Params.Speed(creature) + ";
                            },
                            ArmyGenerationStatParam::Hitpoints => {
                                sort_function += "Creature.Params.Health(creature) + ";
                            },
                        }
                    }
                    sort_function = sort_function
                        .trim_end()
                        .trim_end_matches("+")
                        .trim_end()
                        .to_string();
                    sort_function += "\n\t\t\t\t\treturn result\n\t\t\t\tend)\n\t\t\treturn id";
                    army_generation_rules_script += &format!("{}\n\t\tend,\n", sort_function);   
                }
            }
        }
        script += &format!("{}\t}},\n\n", stack_gen_type_script);
        script += &format!("{}\t}},\n\n", base_army_count_data_script);
        script += &format!("{}\t}},\n\n", army_count_grow_script);
        script += &format!("{}\t}},\n\n", army_generation_rules_script);

        // artifacts script
        if let Some(artifacts_asset) = heroes_repo.get_artifacts_model(asset_id).await? {
            script += "\trequired_artifacts = {";
            for artifact_id in artifacts_asset.required.ids {
                script += &format!("{}, ", artifact_id);
            }
            script.push_str("\t},\n");
            script += "\toptional_artifacts = {\n";
            for (slot, ids) in artifacts_asset.optional.values {
                if !ids.is_empty() {
                    script += &format!("\t\t[{}] = {{", &slot);
                    for id in ids {
                        script += &format!("{}, ", id);
                    }
                    script += "},\n"
                }
            }
            script.push_str("\t},\n\n");

            script += "\tartifacts_base_costs = {\n";
            for (difficulty, cost) in artifacts_asset.base_powers.data {
                script += &format!("\t\t[{}] = {},\n", difficulty, cost);
            }
            script.push_str("\t},\n\n");

            if artifacts_asset.generation_type == AssetGenerationType::Dynamic {
                script += "\tartifacts_costs_grow = {\n";
                for (difficulty, cost) in artifacts_asset.powers_grow.unwrap().data {
                    script += &format!("\t\t[{}] = {},\n", difficulty, cost);
                }
                script.push_str("\t},\n\n");
            }
        }

        script.push('}');
        output_file.write_all(script.as_bytes())?;
    }
    Ok(())
}
