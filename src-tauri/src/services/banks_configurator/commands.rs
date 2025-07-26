use crate::error::Error;
use editor_tools::prelude::*;
use homm5_scaner::prelude::*;
use itertools::Itertools;
use strum::IntoEnumIterator;
use uuid::Uuid;
use std::io::Write;
use tauri::State;

#[tauri::command]
pub async fn get_all_banks(
    bank_service: State<'_, BanksGeneratorRepo>,
) -> Result<Vec<BankDBModel>, Error> {
    Ok(bank_service.get_banks().await?)
}

#[tauri::command]
pub async fn load_bank(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
) -> Result<Option<BankDBModel>, Error> {
    Ok(bank_service.get_bank(id).await?)
}

#[tauri::command]
pub async fn update_bank_recharges_count(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
    count: String,
) -> Result<i32, Error> {
    let actual_count = count.parse::<i32>()?;
    let payload = UpdateBankPayload::new(id).with_recharge_count(actual_count);
    bank_service.update_bank(payload).await?;
    Ok(actual_count)
}

#[tauri::command]
pub async fn update_bank_recharge_timer(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
    timer: String,
) -> Result<i32, Error> {
    let actual_timer = timer.parse::<i32>()?;
    let payload = UpdateBankPayload::new(id).with_recharge_timer(actual_timer);
    bank_service.update_bank(payload).await?;
    Ok(actual_timer)
}

#[tauri::command]
pub async fn update_bank_morale_loss(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
    loss: String,
) -> Result<i32, Error> {
    let actual_loss = loss.parse::<i32>()?;
    let payload = UpdateBankPayload::new(id).with_morale_loss(actual_loss);
    bank_service.update_bank(payload).await?;
    Ok(actual_loss)
}

#[tauri::command]
pub async fn update_bank_luck_loss(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
    loss: String,
) -> Result<i32, Error> {
    let actual_loss = loss.parse::<i32>()?;
    let payload = UpdateBankPayload::new(id).with_luck_loss(actual_loss);
    bank_service.update_bank(payload).await?;
    Ok(actual_loss)
}

#[tauri::command]
pub async fn load_difficulty(
    bank_service: State<'_, BanksGeneratorRepo>,
    bank_id: i32,
    difficulty: BankDifficultyType
) -> Result<Option<BankDifficultyDBModel>, Error> {
    Ok(bank_service.load_difficulty(bank_id, difficulty).await?)
}   

#[tauri::command]
pub async fn update_bank_difficulty_chance(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
    chance: String,
) -> Result<i32, Error> {
    let actual_chance = chance.parse::<i32>()?;
    bank_service.update_difficulty(id, actual_chance).await?;
    Ok(actual_chance)
}

#[tauri::command]
pub async fn load_bank_variants(
    bank_service: State<'_, BanksGeneratorRepo>,
    bank_id: i32,
    difficulty: BankDifficultyType
) -> Result<Vec<BankVariantDBModel>, Error> {
    Ok(bank_service.get_variants(bank_id, difficulty).await?)
}

#[tauri::command]
pub async fn create_bank_variant(
    bank_service: State<'_, BanksGeneratorRepo>,
    bank_id: i32,
    label: String,
    difficulty: BankDifficultyType,
) -> Result<BankVariantDBModel, Error> {
    let new_variant = bank_service
        .create_variant(CreateVariantPayload {
            bank_id,
            label,
            difficulty: difficulty.into(),
        })
        .await?;
    Ok(new_variant)
}

#[tauri::command]
pub async fn load_bank_variant(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: Uuid,
) -> Result<Option<BankVariantDBModel>, Error> {
    if let Some(variant) = bank_service.get_variant(id).await? {
        Ok(Some(variant))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn update_bank_variant_label(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: Uuid,
    label: String,
) -> Result<(), Error> {
    let payload = UpdateBankVariantPayload::new(id).with_label(label);
    bank_service.update_variant(payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_bank_variant_difficulty(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: Uuid,
    difficulty: BankDifficultyType,
) -> Result<(), Error> {
    let payload = UpdateBankVariantPayload::new(id).with_difficulty(difficulty.into());
    bank_service.update_variant(payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn load_creature_slots_ids(
    bank_service: State<'_, BanksGeneratorRepo>,
    variant_id: i32,
) -> Result<Vec<i32>, Error> {
    Ok(bank_service.load_creature_entries(variant_id).await?)
}

#[tauri::command]
pub async fn create_creature_slot(
    bank_service: State<'_, BanksGeneratorRepo>,
    variant_id: i32,
    slot_type: BankCreatureSlotType,
) -> Result<i32, Error> {
    let mut slot_data = CreatureSlotData {
        base_power: Some(0),
        power_grow: Some(0),
        ..Default::default()
    };
    match slot_type {
        BankCreatureSlotType::Tier => {
            slot_data.creature_tier = Some(1);
            slot_data.creature_town = Some(Town::TownNoType);
        }
        BankCreatureSlotType::Concrete => {
            slot_data.creature_id = Some(1);
        }
    }
    let new_id = bank_service
        .create_creature_entry(variant_id, slot_type.into(), slot_data)
        .await?;
    Ok(new_id)
}

#[tauri::command]
pub async fn load_creature_slot(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
) -> Result<Option<CreatureSlotData>, Error> {
    if let Some(slot_data) = bank_service.load_creature_entry(id).await? {
        Ok(Some(slot_data.data))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn update_creature_slot_base_power(
    bank_service: State<'_, BanksGeneratorRepo>,
    slot_id: i32,
    power: String,
) -> Result<i32, Error> {
    let actual_power = power.parse::<i32>()?;
    let payload = UpdateCreatureEntryPayload::new(slot_id).with_base_power(actual_power);
    bank_service.update_creature_entry(payload).await?;
    Ok(actual_power)
}

#[tauri::command]
pub async fn update_creature_slot_power_grow(
    bank_service: State<'_, BanksGeneratorRepo>,
    slot_id: i32,
    grow: String,
) -> Result<i32, Error> {
    let actual_grow = grow.parse::<i32>()?;
    let payload = UpdateCreatureEntryPayload::new(slot_id).with_power_grow(actual_grow);
    bank_service.update_creature_entry(payload).await?;
    Ok(actual_grow)
}

#[tauri::command]
pub async fn update_creature_slot_town(
    bank_service: State<'_, BanksGeneratorRepo>,
    slot_id: i32,
    town: Town,
) -> Result<(), Error> {
    let payload = UpdateCreatureEntryPayload::new(slot_id).with_town(town);
    bank_service.update_creature_entry(payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_creature_slot_tier(
    bank_service: State<'_, BanksGeneratorRepo>,
    slot_id: i32,
    tier: i32,
) -> Result<(), Error> {
    let payload = UpdateCreatureEntryPayload::new(slot_id).with_tier(tier);
    bank_service.update_creature_entry(payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn generate_banks_script(
    bank_service: State<'_, BanksGeneratorRepo>,
    fight_generator_repo: State<'_, FightGeneratorRepo>
) -> Result<(), Error> {
    let banks = bank_service.get_banks().await?;
    for bank in banks {
        let bank_local_type = BankType::from(bank._type.clone());
        let bank_file_name = bank_service.path.join(format!(
            "{}.lua",
            bank_local_type
                .to_string()
                .replace("BTD_BANK_", "")
                .to_lowercase()
        ));
        let mut bank_file = std::fs::File::create(bank_file_name)?;
        // loading data
        let mut bank_data_string = format!(
            "while not {} and BTD_BANKS_DATA do\n\tsleep()\nend\n\n",
            bank_local_type
        );
        // base bank info
        bank_data_string += &format!("BTD_BANKS_DATA[{}] = {{\n", bank_local_type);
        bank_data_string += &format!(
            "\trecharges_count = {},\n\trecharge_timer = {},\n\tmorale_loss = {},\n\tluck_loss = {},\n",
            bank.recharge_count, bank.recharge_timer, bank.morale_loss, bank.luck_loss
        );
        // bank difficulties info
        bank_data_string += "\tgeneration_data = {\n";
        let difficulties = BankDifficultyType::iter().map(|d| d ).collect_vec();
        for difficulty in difficulties {
            if let Some(difficulty_data) = bank_service.load_difficulty(bank.id, difficulty.clone()).await? {
                bank_data_string += &format!("\t\t[{}] = {{\n\t\t\tchance = {},\n", &difficulty, difficulty_data.chance)
            }
            // this difficulty variants info
            let bank_variants = bank_service.get_variants(bank.id, difficulty).await?;
            bank_data_string += "\t\t\tvariants = {\n";
            for variant in bank_variants {
                bank_data_string += "\t\t\t\t{\n\t\t\t\t\tstacks_data = {\n";
                let stacks_assets = fight_generator_repo.get_stacks(variant.id).await?;
                let mut stack_gen_type_script = String::from("\t\t\t\t\t\tstack_count_generation_logic = {\n");
                let mut base_army_count_data_script = String::from("\t\t\t\t\t\tarmy_base_count_data = {\n");
                let mut army_count_grow_script = String::from("\t\t\t\t\t\tarmy_counts_grow = {\n");
                let mut army_generation_rules_script = String::from("\t\t\t\t\t\tarmy_getters = {\n");

                let mut stack_count = 0;
                for asset in stacks_assets {
                    stack_count += 1;

                    stack_gen_type_script += &format!(
                        "\t\t\t\t\t\t\t[{}] = {},\n",
                        stack_count, &asset.count_generation_mode
                    );
                    base_army_count_data_script += &format!("\t\t\t\t\t\t\t[{}] = {{\n", stack_count);

                    if asset.count_generation_mode == ArmySlotStackCountGenerationMode::PowerBased {
                        for (difficulty, power) in asset.base_powers.data {
                            base_army_count_data_script +=
                                &format!("\t\t\t\t\t\t\t\t[{}] = {},\n", &difficulty, power);
                        }
                        base_army_count_data_script += "\t\t\t\t\t\t\t},\n";

                        if asset.power_based_generation_type == AssetGenerationType::Dynamic {
                            army_count_grow_script += &format!("\t\t\t\t\t\t\t[{}] = {{\n", stack_count);
                            for (difficulty, power) in asset.powers_grow.data {
                                army_count_grow_script +=
                                    &format!("\t\t\t\t\t\t\t\t[{}] = {},\n", &difficulty, power);
                            }
                            army_count_grow_script += "\t\t\t\t\t\t\t},\n";
                        }
                    } else {
                        for (difficulty, power) in asset.concrete_count.data {
                            base_army_count_data_script +=
                                &format!("\t\t\t\t\t\t[{}] = {},\n", &difficulty, power);
                        }
                        base_army_count_data_script += "\t\t\t\t\t\t},\n";
                    }

                    // construct generation rule filter
                    if asset.type_generation_mode == ArmySlotStackUnitGenerationMode::ConcreteUnit {
                        let creatures_list = asset.concrete_creatures.ids.iter().join(", ");
                        army_generation_rules_script += &format!(
                            "\t\t\t\t\t\t\t[{}] = function ()
                    local result = Random.FromTable({{{}}})
                    return result
                end,\n",
                            stack_count, creatures_list
                        );
                    } else {
                        army_generation_rules_script += &format!(r#"                            [{}] = function ()"#, stack_count);

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
                            r#"local possible_creatures = Creature.Selection.FromTownsAndTiers({{{}}}, {{{}}})"#,
                            towns, tiers
                        );
                        let filter_function = format!(
                            r#"
                                local filtered_creatures = list_iterator.Filter(
                                    possible_creatures,
                                    function(creature)
                                        {}
                                        return result
                                    end)"#,
                            &generation_rules_script
                        );

                        let stats_elements = fight_generator_repo.get_stat_generation_elements(asset.id).await?;
                        //println!("Stats elements: {:#?}", &stats_elements);
                        if stats_elements.len() == 0 || stats_elements[0].stats.values.len() == 0 {
                            if asset.generation_rule.params.len() > 0 {
                                inner_getter_function += &format!(
                            r#"{}
                                local id = Random.FromTable(filtered_creatures)
                                return id"#, &filter_function);
                            } else {
                                inner_getter_function += r#"
                                local id = Random.FromTable(possible_creatures)
                                return id"#;
                            }
                            army_generation_rules_script +=
                                &format!(r#"
                                {}
                            end,
                        "#, inner_getter_function);
                        } else {
                            let stat_element = &stats_elements[0];
                            let mut sort_function = String::new();
                            // if stat_element.stats.values.len() > 0 {
                            let towns = asset.towns.towns.iter().join(", ");
                            let tiers = asset.tiers.tiers.iter().join(", ");
                            sort_function += &format!(
                                "{}\n\t\t\tlocal id = list_iterator.{}(\n\t\t\t\t{},\n\t\t\t\tfunction(creature)\n\t\t\t\t\tlocal result = ",
                                if asset.generation_rule.params.len() > 0 {
                                    format!(
                                        "\t\t\tlocal possible_creatures = Creature.Selection.FromTownsAndTiers({{{}}}, {{{}}})\n{}",
                                        towns, tiers, filter_function
                                    )
                                } else {
                                    format!(
                                        "\t\t\tlocal possible_creatures = Creature.Selection.FromTownsAndTiers({{{}}}, {{{}}})",
                                        towns, tiers
                                    )
                                },
                                if stat_element.rule == ArmyGenerationStatRule::MaxBy {
                                    "MaxBy"
                                } else {
                                    "MinBy"
                                },
                                if asset.generation_rule.params.len() > 0 {
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
                            army_generation_rules_script += &format!("{}\n\t\tend,\n", sort_function);
                        }
                    }
                }
                bank_data_string += &format!("{}\t\t\t\t\t\t}},\n\n", stack_gen_type_script);
                bank_data_string += &format!("{}\t\t\t\t\t\t}},\n\n", base_army_count_data_script);
                bank_data_string += &format!("{}\t\t\t\t\t\t}},\n\n", army_count_grow_script);
                bank_data_string += &format!("{}}},\n", army_generation_rules_script);
                bank_data_string.push_str("\t\t\t\t\t}\n");
                bank_data_string.push_str("\t\t\t\t},\n");
            }
            bank_data_string.push_str("\t\t\t},\n");
            bank_data_string.push_str("\t\t},\n");
        }
        bank_data_string.push_str("\t}\n}");
        bank_file.write_all(bank_data_string.as_bytes())?;
    }
    Ok(())
}
