use crate::{
    error::Error,
    services::fight_generator::utils::{ArmySlotsConverter, SheetToArmyAssetsConverter},
    utils::LocalAppManager,
};
use editor_tools::prelude::{
    AddGenerationStatElementPayload, AddOptionalArtifactPayload, AddRequiredArtifactPayload,
    AddStackPayload, ArmyGenerationRuleParam, ArmyGenerationStatParam, ArmyGenerationStatRule,
    ArmySlotStackCountGenerationMode, ArmySlotStackUnitGenerationMode, ArmyStatGenerationModel,
    AssetArmySlotModel, AssetArtifactsModel, AssetGenerationType, AssetModel, DifficultyType,
    FightGeneratorRepo, InitAssetArtifactsDataPayload, InitFightAssetPayload,
    RemoveOptionalArtifactPayload, RemoveRequiredArtifactPayload,
    UpdateDifficultyBasedPowerPayload, UpdateFightAssetPayload, UpdateGenerationRulesPayload,
    UpdateGenerationStatElementPayload, UpdateStackBaseDataPayload,
    UpdateStackConcreteCreaturesPayload, UpdateStackTiersPayload, UpdateStackTownsPayload,
};
use homm5_scaner::prelude::{
    AbilityDBModel, ArtifactDBModel, ArtifactSlotType, CreatureDBModel, ScanerService, Town,
};

use itertools::Itertools;
use sheets_connector::service::{SheetId, SheetsConnectorService};
use std::{collections::HashMap, io::Write, path::PathBuf};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

#[tauri::command]
pub async fn load_all_assets(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    map_id: i32,
) -> Result<Vec<AssetModel>, Error> {
    Ok(fight_generator_repo.get_all_assets(map_id).await?)
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
pub async fn init_new_asset(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    sheets_connector_repo: State<'_, SheetsConnectorService>,
    app_manager: State<'_, LocalAppManager>,
    name: String,
    path: String,
    table_name: String,
) -> Result<AssetModel, Error> {
    let base_config_locked = app_manager.base_config.read().await;
    let runtime_config_locked = app_manager.runtime_config.read().await;
    let current_map_id = runtime_config_locked
        .current_selected_map
        .ok_or(Error::UndefinedData("Current map id".to_string()))?;
    let map_data = base_config_locked
        .maps
        .iter()
        .find(|map| map.id == current_map_id)
        .ok_or(Error::UndefinedData("Current map data".to_string()))?;

    let spreadsheet_id = &map_data.fights_spreadsheet_id;
    let created_sheet_id = sheets_connector_repo
        .create_sheet(spreadsheet_id, &name)
        .await
        .map_err(|e| Error::SheetsConnector(Box::new(e)))?;

    let payload = InitFightAssetPayload {
        name,
        path_to_generate: path,
        lua_table_name: table_name,
        mission_id: current_map_id as i32,
        sheet_id: created_sheet_id,
    };
    Ok(fight_generator_repo.init_new_asset(payload).await?)
}

#[tauri::command]
pub async fn load_asset(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    id: Uuid,
) -> Result<Option<AssetModel>, Error> {
    Ok(fight_generator_repo.get_asset(id).await?)
}

#[tauri::command]
pub async fn delete_asset(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    id: Uuid,
) -> Result<(), Error> {
    Ok(fight_generator_repo.delete_asset(id).await?)
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
pub async fn load_abilities_models(
    scaner_repo: State<'_, ScanerService>,
) -> Result<Vec<AbilityDBModel>, Error> {
    Ok(scaner_repo.get_abilities().await?)
}

#[tauri::command]
pub async fn try_load_artifacts_data_for_asset(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: Uuid,
) -> Result<Option<AssetArtifactsModel>, Error> {
    Ok(fight_generator_repo.get_artifacts_model(asset_id).await?)
}

#[tauri::command]
pub async fn create_artifacts_data_for_asset(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: Uuid,
    generation_type: AssetGenerationType,
) -> Result<AssetArtifactsModel, Error> {
    let payload = InitAssetArtifactsDataPayload {
        asset_id,
        generation_type,
    };
    Ok(fight_generator_repo.add_artifacts_model(payload).await?)
}

#[tauri::command]
pub async fn update_artifacts_base_cost(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String,
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    fight_generator_repo
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
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String,
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    fight_generator_repo
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
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: i32,
    artifact_id: i32,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .add_required_artifact_id(AddRequiredArtifactPayload {
            asset_id,
            artifact_id,
        })
        .await?)
}

#[tauri::command]
pub async fn remove_required_artifact(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: i32,
    artifact_id: i32,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .remove_required_artifact_id(RemoveRequiredArtifactPayload {
            asset_id,
            artifact_id,
        })
        .await?)
}

#[tauri::command]
pub async fn add_optional_artifact(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: i32,
    slot: ArtifactSlotType,
    artifact_id: i32,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .add_optional_artifact_id(AddOptionalArtifactPayload {
            asset_id,
            slot,
            artifact_id,
        })
        .await?)
}

#[tauri::command]
pub async fn remove_optional_artifact(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: i32,
    slot: ArtifactSlotType,
    artifact_id: i32,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .remove_optional_artifact_id(RemoveOptionalArtifactPayload {
            asset_id,
            slot,
            artifact_id,
        })
        .await?)
}

#[tauri::command]
pub async fn load_stacks_ids(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: Uuid,
) -> Result<Vec<i32>, Error> {
    Ok(fight_generator_repo.get_stacks_ids(asset_id).await?)
}

#[tauri::command]
pub async fn create_stack(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: Uuid,
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
    Ok(fight_generator_repo.add_stack(payload).await?)
}

#[tauri::command]
pub async fn load_stack(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    stack_id: i32,
) -> Result<Option<AssetArmySlotModel>, Error> {
    Ok(fight_generator_repo.get_stack(stack_id).await?)
}

#[tauri::command]
pub async fn delete_stack(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    stack_id: i32,
) -> Result<(), Error> {
    Ok(fight_generator_repo.delete_stack(stack_id).await?)
}

#[tauri::command]
pub async fn update_stack_data(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    stack_id: i32,
    unit_generation_type: ArmySlotStackUnitGenerationMode,
    count_generation_type: ArmySlotStackCountGenerationMode,
    count_generation_mode: AssetGenerationType,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .update_stack_base_data(UpdateStackBaseDataPayload {
            stack_id,
            unit_generation_type,
            count_generation_type,
            power_based_generation_type: count_generation_mode,
        })
        .await?)
}

#[tauri::command]
pub async fn update_stack_base_powers(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String,
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    fight_generator_repo
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
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String,
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    fight_generator_repo
        .update_stack_power_grow(UpdateDifficultyBasedPowerPayload {
            id: container_id,
            difficulty,
            value: actual_value,
        })
        .await?;
    Ok(actual_value)
}

#[tauri::command]
pub async fn update_stack_concrete_count(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    container_id: i32,
    difficulty: DifficultyType,
    value: String,
) -> Result<i32, Error> {
    let actual_value = value.parse::<i32>()?;
    fight_generator_repo
        .update_stack_creature_count(UpdateDifficultyBasedPowerPayload {
            id: container_id,
            difficulty,
            value: actual_value,
        })
        .await?;
    Ok(actual_value)
}

#[tauri::command]
pub async fn update_stack_concrete_creatures(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    stack_id: i32,
    creatures: Vec<i32>,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .update_stack_concrete_creatures(UpdateStackConcreteCreaturesPayload {
            stack_id,
            creatures,
        })
        .await?)
}

#[tauri::command]
pub async fn update_stack_towns(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    stack_id: i32,
    towns: Vec<Town>,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .update_stack_towns(UpdateStackTownsPayload { stack_id, towns })
        .await?)
}

#[tauri::command]
pub async fn update_stack_tiers(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    stack_id: i32,
    tiers: Vec<i32>,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .update_stack_tiers(UpdateStackTiersPayload { stack_id, tiers })
        .await?)
}

#[tauri::command]
pub async fn update_stack_rules(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    stack_id: i32,
    rules: Vec<ArmyGenerationRuleParam>,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .update_stack_rules(UpdateGenerationRulesPayload { stack_id, rules })
        .await?)
}

#[tauri::command]
pub async fn load_stats_generation_elements(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    stack_id: i32,
) -> Result<Vec<ArmyStatGenerationModel>, Error> {
    Ok(fight_generator_repo
        .get_stat_generation_elements(stack_id)
        .await?)
}

#[tauri::command]
pub async fn add_stat_generation_element(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    stack_id: i32,
    rule: ArmyGenerationStatRule,
) -> Result<ArmyStatGenerationModel, Error> {
    Ok(fight_generator_repo
        .add_stat_generation_element(AddGenerationStatElementPayload { stack_id, rule })
        .await?)
}

#[tauri::command]
pub async fn remove_stat_generation_element(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    element_id: i32,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .delete_stat_generation_element(element_id)
        .await?)
}

#[tauri::command]
pub async fn update_stat_generation_element_priority(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    element_id: i32,
    priority: i32,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .update_stat_generation_element(
            UpdateGenerationStatElementPayload::new(element_id).with_priority(priority),
        )
        .await?)
}

#[tauri::command]
pub async fn update_stat_generation_element_rule(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    element_id: i32,
    rule: ArmyGenerationStatRule,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .update_stat_generation_element(
            UpdateGenerationStatElementPayload::new(element_id).with_rule(rule),
        )
        .await?)
}

#[tauri::command]
pub async fn update_stat_generation_params(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    element_id: i32,
    params: Vec<ArmyGenerationStatParam>,
) -> Result<(), Error> {
    Ok(fight_generator_repo
        .update_stat_generation_element(
            UpdateGenerationStatElementPayload::new(element_id).with_stats(params),
        )
        .await?)
}

#[tauri::command]
pub async fn get_average_creatures_count(
    scaner_repo: State<'_, ScanerService>,
    power: i32,
    towns: Vec<Town>,
    tiers: Vec<i32>,
) -> Result<Option<i32>, Error> {
    Ok(scaner_repo
        .get_average_counts_for_power(power, towns, tiers)
        .await?)
}

#[tauri::command]
pub async fn get_average_concrete_creatures_count(
    scaner_repo: State<'_, ScanerService>,
    power: i32,
    creatures: Vec<i32>,
) -> Result<Option<i32>, Error> {
    Ok(scaner_repo
        .get_average_concrete_creatures_count_for_power(power, creatures)
        .await?)
}

#[tauri::command]
pub async fn get_average_artifacts_cost(
    scaner_repo: State<'_, ScanerService>,
    artifacts: HashMap<ArtifactSlotType, Vec<i32>>,
) -> Result<Option<i32>, Error> {
    let artifacts_list = artifacts.into_values().flatten().collect_vec();
    println!("Artifacts: {:?}", &artifacts_list);
    Ok(scaner_repo
        .get_average_artifacts_cost(artifacts_list)
        .await?)
}

#[tauri::command]
pub async fn generate_current_hero_script(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: Uuid,
) -> Result<(), Error> {
    if let Some(main_asset) = fight_generator_repo.get_asset(asset_id).await? {
        let mut output_file = std::fs::File::create(format!(
            "{}\\{}.lua",
            &main_asset.path_to_generate, main_asset.table_name
        ))?;
        let mut script = format!(
            "
while not UNIT_COUNT_GENERATION_MODE_POWER_BASED and not UNIT_COUNT_GENERATION_MODE_RAW do
    sleep()
end

{} = {{\n",
            &main_asset.table_name
        );
        // stacks script
        let stacks_assets = fight_generator_repo.get_stacks(main_asset.id).await?;
        let mut stack_gen_type_script = String::from("\tstack_count_generation_logic = {\n");
        let mut base_army_count_data_script = String::from("\tarmy_base_count_data = {\n");
        let mut army_count_grow_script = String::from("\tarmy_counts_grow = {\n");
        let mut army_generation_rules_script = String::from("\tarmy_getters = {\n");

        let mut stack_count = 0;
        for asset in stacks_assets {
            stack_count += 1;

            stack_gen_type_script += &format!(
                "\t\t[{}] = {},\n",
                stack_count, &asset.count_generation_mode
            );
            base_army_count_data_script += &format!("\t\t[{stack_count}] = {{\n");

            if asset.count_generation_mode == ArmySlotStackCountGenerationMode::PowerBased {
                for (difficulty, power) in asset.base_powers.data {
                    base_army_count_data_script +=
                        &format!("\t\t\t[{}] = {},\n", &difficulty, power);
                }
                base_army_count_data_script += "\t\t},\n";

                if asset.power_based_generation_type == AssetGenerationType::Dynamic {
                    army_count_grow_script += &format!("\t\t[{stack_count}] = {{\n");
                    for (difficulty, power) in asset.powers_grow.data {
                        army_count_grow_script +=
                            &format!("\t\t\t[{}] = {},\n", &difficulty, power);
                    }
                    army_count_grow_script += "\t\t},\n";
                }
            } else {
                for (difficulty, power) in asset.concrete_count.data {
                    base_army_count_data_script +=
                        &format!("\t\t\t[{}] = {},\n", &difficulty, power);
                }
                base_army_count_data_script += "\t\t},\n";
            }

            // construct generation rule filter
            if asset.type_generation_mode == ArmySlotStackUnitGenerationMode::ConcreteUnit {
                let creatures_list = asset.concrete_creatures.ids.iter().join(", ");
                army_generation_rules_script += &format!(
                    "\t\t[{stack_count}] = function ()
            local result = Random.FromTable({{{creatures_list}}})
            return result
        end,\n"
                );
            } else {
                army_generation_rules_script += &format!("\t\t[{stack_count}] = function ()\n");

                let mut generation_rules_script = String::from("local result = ");
                for rule in &asset.generation_rule.params {
                    match rule {
                        ArmyGenerationRuleParam::Generatable => {
                            generation_rules_script +=
                                "Creature.Params.IsGeneratable(creature) and "
                        }
                        ArmyGenerationRuleParam::Caster => {
                            generation_rules_script += "Creature.Type.IsCaster(creature) and "
                        }
                        ArmyGenerationRuleParam::Shooter => {
                            generation_rules_script += "Creature.Type.IsShooter(creature) and "
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
                let towns = asset.towns.towns.iter().join(", ");
                let tiers = asset.tiers.tiers.iter().join(", ");
                let mut inner_getter_function = format!(
                    "\t\t\tlocal possible_creatures = Creature.Selection.FromTownsAndTiers({{{towns}}}, {{{tiers}}})\n"
                );
                let filter_function = format!(
                    "\t\t\tlocal filtered_creatures = list_iterator.Filter(\n\t\t\t\tpossible_creatures,\n\t\t\t\tfunction(creature)\n\t\t\t\t\t{}\n\t\t\t\t\treturn result\n\t\t\t\tend)",
                    &generation_rules_script
                );

                let stats_elements = fight_generator_repo
                    .get_stat_generation_elements(asset.id)
                    .await?;
                //println!("Stats elements: {:#?}", &stats_elements);
                if stats_elements.is_empty() || stats_elements[0].stats.values.is_empty() {
                    if !asset.generation_rule.params.is_empty() {
                        inner_getter_function += &format!(
                            "{}\n\t\t\tlocal id = Random.FromTable(filtered_creatures)\n\t\t\treturn id",
                            &filter_function
                        );
                    } else {
                        inner_getter_function += "\t\t\tlocal id = Random.FromTable(possible_creatures)\n\t\t\treturn id";
                    }
                    army_generation_rules_script += &format!("{inner_getter_function}\n\t\tend,\n");
                } else {
                    let stat_element = &stats_elements[0];
                    let mut sort_function = String::new();
                    // if stat_element.stats.values.len() > 0 {
                    let towns = asset.towns.towns.iter().join(", ");
                    let tiers = asset.tiers.tiers.iter().join(", ");
                    sort_function += &format!(
                        "{}\n\t\t\tlocal id = list_iterator.{}(\n\t\t\t\t{},\n\t\t\t\tfunction(creature)\n\t\t\t\t\tlocal result = ",
                        if !asset.generation_rule.params.is_empty() {
                            format!(
                                "\t\t\tlocal possible_creatures = Creature.Selection.FromTownsAndTiers({{{towns}}}, {{{tiers}}})\n{filter_function}"
                            )
                        } else {
                            format!(
                                "\t\t\tlocal possible_creatures = Creature.Selection.FromTownsAndTiers({{{towns}}}, {{{tiers}}})"
                            )
                        },
                        if stat_element.rule == ArmyGenerationStatRule::MaxBy {
                            "MaxBy"
                        } else {
                            "MinBy"
                        },
                        if !asset.generation_rule.params.is_empty() {
                            "filtered_creatures"
                        } else {
                            "possible_creatures"
                        }
                    );
                    for param in &stat_element.stats.values {
                        match param {
                            ArmyGenerationStatParam::Attack => {
                                sort_function += "Creature.Params.Attack(creature) + ";
                            }
                            ArmyGenerationStatParam::Defence => {
                                sort_function += "Creature.Params.Defence(creature) + ";
                            }
                            ArmyGenerationStatParam::Initiative => {
                                sort_function += "Creature.Params.Initiative(creature) + ";
                            }
                            ArmyGenerationStatParam::Speed => {
                                sort_function += "Creature.Params.Speed(creature) + ";
                            }
                            ArmyGenerationStatParam::Hitpoints => {
                                sort_function += "Creature.Params.Health(creature) + ";
                            }
                        }
                    }
                    sort_function = sort_function
                        .trim_end()
                        .trim_end_matches("+")
                        .trim_end()
                        .to_string();
                    sort_function += "\n\t\t\t\t\treturn result\n\t\t\t\tend)\n\t\t\treturn id";
                    // }
                    army_generation_rules_script += &format!("{sort_function}\n\t\tend,\n");
                }
            }
        }
        script += &format!("{stack_gen_type_script}\t}},\n\n");
        script += &format!("{base_army_count_data_script}\t}},\n\n");
        script += &format!("{army_count_grow_script}\t}},\n\n");
        script += &format!("{army_generation_rules_script}\t}},\n\n");

        // artifacts script
        if let Some(artifacts_asset) = fight_generator_repo.get_artifacts_model(asset_id).await? {
            script += "\trequired_artifacts = {";
            for artifact_id in artifacts_asset.required.ids {
                script += &format!("{artifact_id}, ");
            }
            script.push_str("\t},\n");
            script += "\toptional_artifacts = {\n";
            for (slot, ids) in artifacts_asset.optional.values {
                if !ids.is_empty() {
                    script += &format!("\t\t[{}] = {{{}}},\n", &slot, ids.iter().join(", "));
                }
            }
            script.push_str("\t},\n\n");

            script += "\tartifacts_base_costs = {\n";
            for (difficulty, cost) in artifacts_asset.base_powers.data {
                script += &format!("\t\t[{difficulty}] = {cost},\n");
            }
            script.push_str("\t},\n\n");

            if artifacts_asset.generation_type == AssetGenerationType::Dynamic {
                script += "\tartifacts_costs_grow = {\n";
                for (difficulty, cost) in artifacts_asset.powers_grow.unwrap().data {
                    script += &format!("\t\t[{difficulty}] = {cost},\n");
                }
                script.push_str("\t},\n\n");
            }
        }

        script.push('}');
        output_file.write_all(script.as_bytes())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn create_sheet_for_existing_asset(
    scaner_repo: State<'_, ScanerService>,
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    sheets_connector_repo: State<'_, SheetsConnectorService>,
    app_manager: State<'_, LocalAppManager>,
    asset_id: Uuid,
) -> Result<SheetId, Error> {
    let base_config_locked = app_manager.base_config.read().await;
    let asset = fight_generator_repo
        .get_asset(asset_id)
        .await?
        .ok_or(Error::UndefinedData("Asset to sync".to_string()))?;
    let creatures = scaner_repo.get_all_creature_models().await?;
    let spreadsheet_id = &base_config_locked
        .maps
        .iter()
        .find(|map| (map.id as i32) == asset.mission_id)
        .ok_or(Error::UndefinedData(String::from("Current map")))?
        .fights_spreadsheet_id;

    let created_sheet_id = sheets_connector_repo
        .create_sheet(spreadsheet_id, &asset.name)
        .await
        .map_err(|e| Error::SheetsConnector(Box::new(e)))?;
    let army_slots = fight_generator_repo.get_stacks(asset_id).await?;
    let stat_elements = fight_generator_repo
        .get_all_stat_elements_for_stacks(army_slots.iter().map(|a| a.id).collect_vec())
        .await?;
    let converter = ArmySlotsConverter {
        sheet_name: &asset.name,
        creatures_data: &creatures,
        stats_elements_data: &stat_elements,
    };
    sheets_connector_repo
        .upload_to_sheet(spreadsheet_id, army_slots, converter)
        .await
        .map_err(|e| Error::SheetsConnector(Box::new(e)))?;

    fight_generator_repo
        .update_asset(UpdateFightAssetPayload::new(asset_id).with_sheet_id(created_sheet_id))
        .await?;
    Ok(created_sheet_id)
}

#[tauri::command]
pub async fn pull_from_sheet(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    sheets_connector_repo: State<'_, SheetsConnectorService>,
    app_manager: State<'_, LocalAppManager>,
    asset_id: Uuid,
) -> Result<Vec<AssetArmySlotModel>, Error> {
    let base_config_locked = app_manager.base_config.read().await;
    let asset = fight_generator_repo
        .get_asset(asset_id)
        .await?
        .ok_or(Error::UndefinedData("Asset to sync".to_string()))?;
    let stacks_count = fight_generator_repo
        .get_stacks_count_for_asset(asset_id)
        .await?;

    let spreadsheet_id = &base_config_locked
        .maps
        .iter()
        .find(|map| (map.id as i32) == asset.mission_id)
        .ok_or(Error::UndefinedData(String::from("Current map")))?
        .fights_spreadsheet_id;

    let converter = SheetToArmyAssetsConverter::new(asset_id);
    let values = sheets_connector_repo
        .read_from_sheet(
            spreadsheet_id,
            asset.sheet_id.unwrap(),
            &format!(
                "B2:{}",
                match stacks_count {
                    1 => "B24",
                    2 => "C24",
                    3 => "D24",
                    4 => "E24",
                    5 => "F24",
                    6 => "G24",
                    7 => "H24",
                    _ => unreachable!(),
                }
            ),
            converter,
        )
        .await
        .map_err(|e| Error::SheetsConnector(Box::new(e)))?;

    println!("Got values from sheet: {:#?}", &values);
    let updated_slots = fight_generator_repo
        .update_synced_army_slots(values)
        .await?;
    Ok(updated_slots)
}

#[tauri::command]
pub async fn push_to_sheet(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    sheets_connector_repo: State<'_, SheetsConnectorService>,
    scaner_repo: State<'_, ScanerService>,
    app_manager: State<'_, LocalAppManager>,
    asset_id: Uuid,
) -> Result<(), Error> {
    let base_config_locked = app_manager.base_config.read().await;
    let asset = fight_generator_repo
        .get_asset(asset_id)
        .await?
        .ok_or(Error::UndefinedData("Asset to sync".to_string()))?;
    let creatures = scaner_repo.get_all_creature_models().await?;
    let spreadsheet_id = &base_config_locked
        .maps
        .iter()
        .find(|map| (map.id as i32) == asset.mission_id)
        .ok_or(Error::UndefinedData(String::from("Current map")))?
        .fights_spreadsheet_id;

    let army_slots = fight_generator_repo.get_stacks(asset_id).await?;
    let stat_elements = fight_generator_repo
        .get_all_stat_elements_for_stacks(army_slots.iter().map(|a| a.id).collect_vec())
        .await?;
    let converter = ArmySlotsConverter {
        sheet_name: &asset.name,
        creatures_data: &creatures,
        stats_elements_data: &stat_elements,
    };
    sheets_connector_repo
        .upload_to_sheet(spreadsheet_id, army_slots, converter)
        .await
        .map_err(|e| Error::SheetsConnector(Box::new(e)))?;
    Ok(())
}

#[tauri::command]
pub async fn add_artifacts_data_to_asset_sheet(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    sheets_connector_repo: State<'_, SheetsConnectorService>,
    app_manager: State<'_, LocalAppManager>,
    asset_id: Uuid,
    art_asset_id: i32,
) -> Result<i32, Error> {
    let base_config_locked = app_manager.base_config.read().await;
    let asset = fight_generator_repo
        .get_asset(asset_id)
        .await?
        .ok_or(Error::UndefinedData("Asset to sync".to_string()))?;
    let spreadsheet_id = &base_config_locked
        .maps
        .iter()
        .find(|map| (map.id as i32) == asset.mission_id)
        .ok_or(Error::UndefinedData(String::from("Current map")))?
        .fights_spreadsheet_id;
    sheets_connector_repo
        .update_sheet("addArtifactsData", &serde_json::json!({"spreadsheetId": spreadsheet_id, "sheetId": asset.sheet_id.unwrap()}))
        .await
        .map_err(|e| Error::SheetsConnector(Box::new(e)))?;
    fight_generator_repo.update_artifacts_asset(art_asset_id, asset.sheet_id.unwrap()).await?;
    Ok(asset.sheet_id.unwrap())
}