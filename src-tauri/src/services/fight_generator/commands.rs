use crate::{error::Error, utils::LocalAppManager};
use editor_tools::prelude::{
    AddOptionalArtifactPayload, AddRequiredArtifactPayload, AddStackPayload, AssetArmySlotModel,
    AssetArtifactsModel, AssetModel, AssetUpdateType, CalculationRule, DifficultyType,
    EditorToolsError, FightGeneratorRepo, GetAssetsRequest, InitAssetArtifactsDataPayload,
    InitFightAssetPayload, RemoveOptionalArtifactPayload, RemoveRequiredArtifactPayload,
    SelectionType, StackGenerationRule, StackStatsRuleModel, StatParam, StatSelectionRule,
    UpdateDifficultyBasedPowerPayload, UpdateGenerationRulesPayload, UpdateStackBaseDataPayload,
    UpdateStackSpecificCreaturesPayload, UpdateStackTiersPayload, UpdateStackTownsPayload,
    UpdateStatRulesPayload,
};
use homm5_scaner::prelude::{
    AbilityDBModel, ArtifactDBModel, ArtifactSlotType, CreatureDBModel, GetArtifactsRequest,
    GetCreaturesRequest, ScanerService, SpellDBModel, TownExtended,
};

use crate::services::version_control::VersionControlService;
use google_api::service::GoogleApiService;
use itertools::Itertools;
use std::{collections::HashMap, io::Write, path::PathBuf};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn load_all_assets(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    map_id: i32,
) -> Result<Vec<AssetModel>, Error> {
    Ok(fight_generator_repo
        .get_assets(GetAssetsRequest::OfMission(map_id))
        .await?)
}

#[tauri::command]
pub async fn pick_hero_lua_generation_directory(
    app: AppHandle,
    app_manager: State<'_, LocalAppManager>,
) -> Result<(), ()> {
    let profile = app_manager.current_profile_data.read().await;
    let current_map_id = app_manager.runtime_config.read().await.current_selected_map;
    let map = profile
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
    google_api: State<'_, GoogleApiService>,
    app_manager: State<'_, LocalAppManager>,
    version_control: State<'_, VersionControlService>,
    name: String,
    path: String,
    table_name: String,
) -> Result<AssetModel, Error> {
    let runtime_config_locked = app_manager.runtime_config.read().await;
    let profile = app_manager.current_profile_data.read().await;
    let current_map_id = runtime_config_locked.current_selected_map;

    let spreadsheet_id = profile.get_map_spreadsheet_id(current_map_id);

    google_api
        .create_sheet(&spreadsheet_id, &table_name)
        .await
        .map_err(|e| Error::GoogleApi(Box::new(e)))?;

    let payload = InitFightAssetPayload {
        app_name: name,
        path_to_generate: path,
        lua_table_name: table_name,
        mission_id: current_map_id,
    };
    match fight_generator_repo.init_new_asset(payload).await {
        Ok(asset) => {
            version_control.updated_fights.insert(asset.id);
            Ok(asset)
        }
        Err(e) => Err(Error::EditorTools(e))
    }
}

#[tauri::command]
pub async fn load_asset(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    id: i32,
) -> Result<Option<AssetModel>, Error> {
    Ok(fight_generator_repo.get_asset(id).await?)
}

#[tauri::command]
pub async fn delete_asset(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    id: i32,
) -> Result<(), Error> {
    Ok(fight_generator_repo.delete_asset(id).await?)
}

#[tauri::command]
pub async fn load_artifact_models(
    scaner_repo: State<'_, ScanerService>,
) -> Result<Vec<ArtifactDBModel>, Error> {
    Ok(scaner_repo
        .get_artifact_models(GetArtifactsRequest::Any)
        .await?)
}

#[tauri::command]
pub async fn load_creature_models(
    scaner_repo: State<'_, ScanerService>,
) -> Result<Vec<CreatureDBModel>, Error> {
    Ok(scaner_repo
        .get_creature_models(GetCreaturesRequest::NonDefault)
        .await?)
}

#[tauri::command]
pub async fn load_abilities_models(
    scaner_repo: State<'_, ScanerService>,
) -> Result<Vec<AbilityDBModel>, Error> {
    Ok(scaner_repo.get_abilities().await?)
}

#[tauri::command]
pub async fn load_spells_models(
    scaner_repo: State<'_, ScanerService>,
) -> Result<Vec<SpellDBModel>, Error> {
    Ok(scaner_repo.get_all_spells().await?)
}

#[tauri::command]
pub async fn try_load_artifacts_data_for_asset(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: i32,
) -> Result<Option<AssetArtifactsModel>, Error> {
    Ok(fight_generator_repo.get_artifacts_model(asset_id).await?)
}

#[tauri::command]
pub async fn create_artifacts_data_for_asset(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    google_api: State<'_, GoogleApiService>,
    app_manager: State<'_, LocalAppManager>,
    version_control: State<'_, VersionControlService>,
    asset_id: i32,
    generation_type: AssetUpdateType,
) -> Result<AssetArtifactsModel, Error> {
    let profile = app_manager.current_profile_data.read().await;
    let asset = fight_generator_repo
        .get_asset(asset_id)
        .await?
        .ok_or(Error::UndefinedData("Asset to sync".to_string()))?;
    let spreadsheet_id = &profile.get_map_spreadsheet_id(asset.mission_id);
    google_api
        .update_sheet(
            "addArtifactsData",
            &serde_json::json!({"spreadsheetId": spreadsheet_id, "sheetName": asset.table_name}),
        )
        .await
        .map_err(|e| Error::GoogleApi(Box::new(e)))?;
    match fight_generator_repo
        .add_artifacts_model(InitAssetArtifactsDataPayload {
            asset_id,
            update_type: generation_type,
        })
        .await
    {
        Ok(model) => {
            version_control.updated_fights.insert(asset.id);
            Ok(model)
        }
        Err(e) => Err(Error::EditorTools(e))
    }
}

#[tauri::command]
pub async fn update_artifacts_base_cost(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    asset_id: i32,
    difficulty: DifficultyType,
    value: i32,
) -> Result<i32, Error> {
    fight_generator_repo
        .update_artifacts_base_generation_power(UpdateDifficultyBasedPowerPayload {
            id: asset_id,
            difficulty,
            value,
        })
        .await?;
    version_control.updated_fights.insert(asset_id);
    Ok(value)
}

#[tauri::command]
pub async fn update_artifacts_cost_grow(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    asset_id: i32,
    difficulty: DifficultyType,
    value: i32,
) -> Result<i32, Error> {
    fight_generator_repo
        .update_artifacts_grow_power(UpdateDifficultyBasedPowerPayload {
            id: asset_id,
            difficulty,
            value,
        })
        .await?;
    version_control.updated_fights.insert(asset_id);
    Ok(value)
}

#[tauri::command]
pub async fn add_required_artifact(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    asset_id: i32,
    artifact_id: i32,
) -> Result<(), Error> {
    match fight_generator_repo
        .add_required_artifact_id(AddRequiredArtifactPayload {
            asset_id,
            artifact_id,
        })
        .await
    {
        Ok(_) => {
            version_control.updated_fights.insert(asset_id);
            Ok(())
        }
        Err(e) => Err(Error::EditorTools(e))
    }
}

#[tauri::command]
pub async fn remove_required_artifact(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    asset_id: i32,
    artifact_id: i32,
) -> Result<(), Error> {
    match fight_generator_repo
        .remove_required_artifact_id(RemoveRequiredArtifactPayload {
            asset_id,
            artifact_id,
        })
        .await
    {
        Ok(_) => {
            version_control.updated_fights.remove(&asset_id);
            Ok(())
        }
        Err(e) => Err(Error::EditorTools(e))
    }
}

#[tauri::command]
pub async fn add_optional_artifact(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    asset_id: i32,
    slot: ArtifactSlotType,
    artifact_id: i32,
) -> Result<(), Error> {
    match fight_generator_repo
        .add_optional_artifact_id(AddOptionalArtifactPayload {
            asset_id,
            slot,
            artifact_id,
        })
        .await
    {
        Ok(_) => {
            version_control.updated_fights.insert(asset_id);
            Ok(())
        }
        Err(e) => Err(Error::EditorTools(e))
    }
}

#[tauri::command]
pub async fn remove_optional_artifact(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    asset_id: i32,
    slot: ArtifactSlotType,
    artifact_id: i32,
) -> Result<(), Error> {
    match fight_generator_repo
        .remove_optional_artifact_id(RemoveOptionalArtifactPayload {
            asset_id,
            slot,
            artifact_id,
        })
        .await
    {
        Ok(_) => {
            version_control.updated_fights.insert(asset_id);
            Ok(())
        }
        Err(e) => Err(Error::EditorTools(e))
    }
}

#[tauri::command]
pub async fn load_stacks_ids(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: i32,
) -> Result<Vec<i32>, Error> {
    Ok(fight_generator_repo.get_stacks_ids(asset_id).await?)
}

#[tauri::command]
pub async fn create_stack(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    asset_id: i32,
    selection_type: SelectionType,
    calculation_rule: CalculationRule,
    update_type: AssetUpdateType,
) -> Result<i32, Error> {
    let payload = AddStackPayload {
        asset_id,
        selection_rule: selection_type,
        calculation_rule,
        update_type,
    };
    match fight_generator_repo.add_stack(payload).await {
        Ok(stack_id) => {
            version_control.updated_fights.insert(asset_id);
            Ok(stack_id)
        }
        Err(e) => Err(Error::EditorTools(e))
    }
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
    version_control: State<'_, VersionControlService>,
    stack_id: i32,
) -> Result<(), Error> {
    if let Some(asset_id) = fight_generator_repo.delete_stack(stack_id).await? {
        version_control.updated_fights.insert(asset_id);
        Ok(())
    } else {
        Err(Error::EditorTools(EditorToolsError::Default))
    }
}

#[tauri::command]
pub async fn update_stack_data(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    stack_id: i32,
    selection_type: SelectionType,
    calculation_rule: CalculationRule,
    update_type: AssetUpdateType,
) -> Result<(), Error> {
    if let Some(asset_id) = fight_generator_repo
        .update_stack_base_data(UpdateStackBaseDataPayload {
            stack_id,
            selection_rule: selection_type,
            calculation_rule,
            update_type,
        })
        .await?
    {
        version_control.updated_fights.insert(asset_id);
        Ok(())
    } else {
        Err(Error::EditorTools(EditorToolsError::Default))
    }
}

#[tauri::command]
pub async fn update_stack_base_powers(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    container_id: i32,
    difficulty: DifficultyType,
    value: i32,
) -> Result<i32, Error> {
    if let Some(asset_id) = fight_generator_repo
        .update_stack_base_power(UpdateDifficultyBasedPowerPayload {
            id: container_id,
            difficulty,
            value,
        })
        .await?
    {
        version_control.updated_fights.insert(asset_id);
        Ok(value)
    } else {
        Err(Error::EditorTools(EditorToolsError::Default))
    }
}

#[tauri::command]
pub async fn update_stack_powers_grow(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    container_id: i32,
    difficulty: DifficultyType,
    value: i32,
) -> Result<i32, Error> {
    if let Some(asset_id) = fight_generator_repo
        .update_stack_power_grow(UpdateDifficultyBasedPowerPayload {
            id: container_id,
            difficulty,
            value,
        })
        .await?
    {
        version_control.updated_fights.insert(asset_id);
        Ok(value)
    } else {
        Err(Error::EditorTools(EditorToolsError::Default))
    }
}

#[tauri::command]
pub async fn update_stack_concrete_count(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    container_id: i32,
    difficulty: DifficultyType,
    value: i32,
) -> Result<i32, Error> {
    if let Some(asset_id) = fight_generator_repo
        .update_stack_creature_count(UpdateDifficultyBasedPowerPayload {
            id: container_id,
            difficulty,
            value,
        })
        .await?
    {
        version_control.updated_fights.insert(asset_id);
        Ok(value)
    } else {
        Err(Error::EditorTools(EditorToolsError::Default))
    }
}

#[tauri::command]
pub async fn update_stack_concrete_creatures(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    stack_id: i32,
    creatures: Vec<i32>,
) -> Result<(), Error> {
    if let Some(asset_id) = fight_generator_repo
        .update_stack_specific_creatures(UpdateStackSpecificCreaturesPayload {
            stack_id,
            creatures,
        })
        .await?
    {
        version_control.updated_fights.insert(asset_id);
        Ok(())
    } else {
        Err(Error::EditorTools(EditorToolsError::Default))
    }
}

#[tauri::command]
pub async fn update_stack_towns(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    stack_id: i32,
    towns: Vec<TownExtended>,
) -> Result<(), Error> {
    if let Some(asset_id) = fight_generator_repo
        .update_stack_towns(UpdateStackTownsPayload { stack_id, towns })
        .await?
    {
        version_control.updated_fights.insert(asset_id);
        Ok(())
    } else {
        Err(Error::EditorTools(EditorToolsError::Default))
    }
}

#[tauri::command]
pub async fn update_stack_tiers(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    stack_id: i32,
    tiers: Vec<i32>,
) -> Result<(), Error> {
    if let Some(asset_id) = fight_generator_repo
        .update_stack_tiers(UpdateStackTiersPayload { stack_id, tiers })
        .await?
    {
        version_control.updated_fights.insert(asset_id);
        Ok(())
    } else {
        Err(Error::EditorTools(EditorToolsError::Default))
    }
}

#[tauri::command]
pub async fn update_stack_rules(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    stack_id: i32,
    rules: Vec<StackGenerationRule>,
) -> Result<(), Error> {
    if let Some(asset_id) = fight_generator_repo
        .update_stack_rules(UpdateGenerationRulesPayload { stack_id, rules })
        .await?
    {
        version_control.updated_fights.insert(asset_id);
        Ok(())
    } else {
        Err(Error::EditorTools(EditorToolsError::Default))
    }
}

#[tauri::command]
pub async fn update_stack_stat_rules(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    version_control: State<'_, VersionControlService>,
    stack_id: i32,
    rules: Vec<StackStatsRuleModel>,
) -> Result<(), Error> {
    if let Some(asset_id) = fight_generator_repo
        .update_stat_rules(UpdateStatRulesPayload { stack_id, rules })
        .await?
    {
        version_control.updated_fights.insert(asset_id);
        Ok(())
    } else {
        Err(Error::EditorTools(EditorToolsError::Default))
    }
}

#[tauri::command]
pub async fn get_average_creatures_count(
    scaner_repo: State<'_, ScanerService>,
    power: i32,
    towns: Vec<TownExtended>,
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
    Ok(scaner_repo
        .get_average_artifacts_cost(artifacts_list)
        .await?)
}

#[tauri::command]
pub async fn generate_current_hero_script(
    fight_generator_repo: State<'_, FightGeneratorRepo>,
    asset_id: i32,
) -> Result<(), Error> {
    if let Some(main_asset) = fight_generator_repo.get_asset(asset_id).await? {
        let mut output_file = std::fs::File::create(format!(
            "{}\\{}.lua",
            &main_asset.path_to_generate, main_asset.table_name
        ))?;
        let mut script = format!(
            "
while not (UNIT_COUNT_GENERATION_MODE_POWER_BASED and UNIT_COUNT_GENERATION_MODE_RAW and Iterator and GeneratedCombat) do
    sleep()
end

---@type GeneratedCombat
{} = GeneratedCombat({{\n",
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

            stack_gen_type_script +=
                &format!("\t\t[{}] = {},\n", stack_count, &asset.calculation_type);
            base_army_count_data_script += &format!("\t\t[{stack_count}] = {{\n");

            if asset.calculation_type == CalculationRule::PowerBased {
                for (difficulty, power) in asset.base_powers.data {
                    base_army_count_data_script +=
                        &format!("\t\t\t[{}] = {},\n", &difficulty, power);
                }
                base_army_count_data_script += "\t\t},\n";

                if asset.update_type == AssetUpdateType::Dynamic {
                    army_count_grow_script += &format!("\t\t[{stack_count}] = {{\n");
                    for (difficulty, power) in asset.powers_grow.data {
                        army_count_grow_script +=
                            &format!("\t\t\t[{}] = {},\n", &difficulty, power);
                    }
                    army_count_grow_script += "\t\t},\n";
                }
            } else {
                for (difficulty, power) in asset.specific_count.data {
                    base_army_count_data_script +=
                        &format!("\t\t\t[{}] = {},\n", &difficulty, power);
                }
                base_army_count_data_script += "\t\t},\n";
            }

            // construct generation rule filter
            if asset.selection_type == SelectionType::Manual {
                let creatures_list = asset.specific_creatures.ids.iter().join(", ");
                army_generation_rules_script += &format!(
                    "\t\t[{stack_count}] = function ()
            local id = Random.FromTable({{{creatures_list}}})
            local result = Creature(id)
            return result
        end,\n"
                );
            } else {
                army_generation_rules_script += &format!("\t\t[{stack_count}] = function ()\n");

                let mut generation_rules_script = String::from("local result = ");
                if asset.rules.rules.is_empty() {
                    generation_rules_script += "1"
                } else {
                    for rule in &asset.rules.rules {
                        match rule {
                            StackGenerationRule::Generatable => {
                                generation_rules_script += "creature.is_generatable and "
                            }
                            StackGenerationRule::Caster => {
                                generation_rules_script += "creature:IsCaster() and "
                            }
                            StackGenerationRule::Shooter => {
                                generation_rules_script += "creature:IsShooter() and "
                            }
                            _ => {}
                        }
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
                let stats_elements = asset.stat_rules.rules;
                let mut getter_function = format!(
                    r#"            local id = CREATURES_ITERATOR.Filter(
                   ---@param item Creature
                   function(item)
                      local result = contains({{{towns}}}, item.town) and contains({{{tiers}}}, item.tier)
                      return result
                   end)
                   .Filter(
                   ---@param creature Creature
                   function(creature)
                       {generation_rules_script}
                       return result
                   end)"#
                );
                if stats_elements.is_empty() {
                    getter_function += r#"     
                .TakeRandom(1)
                .Collect()[1]
            return id"#;
                    army_generation_rules_script += &format!("{getter_function}\n\t\tend,\n");
                } else {
                    let stat_element = &stats_elements[0];
                    let mut sort_expression = String::from("local result = ");
                    match stat_element.param {
                        StatParam::Attack => {
                            sort_expression += "creature.attack + ";
                        }
                        StatParam::Defence => {
                            sort_expression += "creature.defence + ";
                        }
                        StatParam::Initiative => {
                            sort_expression += "creature.initiative + ";
                        }
                        StatParam::Speed => {
                            sort_expression += "creature.speed + ";
                        }
                        StatParam::Hitpoints => {
                            sort_expression += "creature.health + ";
                        }
                    }
                    sort_expression = sort_expression
                        .trim_end()
                        .trim_end_matches("+")
                        .trim_end()
                        .to_string();

                    getter_function += &format!(
                        r#"
                .{}(function(creature)
                    {}
                    return result
                end)
            return id"#,
                        if stat_element.rule == StatSelectionRule::MaxBy {
                            "MaxBy"
                        } else {
                            "MinBy "
                        },
                        sort_expression
                    );
                    army_generation_rules_script += &format!("{getter_function}\n\t\tend,\n");
                }
            }
        }
        script += &format!("{stack_gen_type_script}\t}},\n\n");
        script += &format!("{base_army_count_data_script}\t}},\n\n");
        script += &format!("{army_count_grow_script}\t}},\n\n");
        script += &format!("{army_generation_rules_script}\t}},\n\n");

        // artifacts script
        if let Some(artifacts_asset) = fight_generator_repo.get_artifacts_model(asset_id).await? {
            script += &format!(
                "\trequired_artifacts = Iterator({{{}}}).Map(function(a) local result = Artifact(a) return result end).Collect(),\n",
                artifacts_asset.required.ids.iter().join(",")
            );
            script += "\toptional_artifacts = {\n";
            for (slot, ids) in artifacts_asset.optional.values {
                if !ids.is_empty() {
                    script += &format!(
                        "\t\t[{}] = Iterator({{{}}}).Map(function(a) local result = Artifact(a) return result end).Collect(),\n",
                        &slot,
                        ids.iter().join(", ")
                    );
                }
            }
            script.push_str("\t},\n\n");

            script += "\tartifacts_base_costs = {\n";
            for (difficulty, cost) in artifacts_asset.base_powers.data {
                script += &format!("\t\t[{difficulty}] = {cost},\n");
            }
            script.push_str("\t},\n\n");

            if artifacts_asset.update_type == AssetUpdateType::Dynamic {
                script += "\tartifacts_costs_grow = {\n";
                for (difficulty, cost) in artifacts_asset.powers_grow.data {
                    script += &format!("\t\t[{difficulty}] = {cost},\n");
                }
                script.push_str("\t},\n\n");
            }
        }

        script.push_str("})\n\n __end_import()");
        output_file.write_all(script.as_bytes())?;
    }
    Ok(())
}
