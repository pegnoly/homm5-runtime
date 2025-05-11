use std::io::Write;
use itertools::Itertools;
use tauri::State;
use editor_tools::prelude::*;
use homm5_scaner::prelude::*;
use crate::{error::Error, services::banks_configurator::types::{BankDifficultyModel, BankSimpleModel}};

#[tauri::command]
pub async fn get_all_banks(
    bank_service: State<'_, BanksGeneratorRepo>
) -> Result<Vec<BankSimpleModel>, Error> {
    let banks = bank_service.get_banks().await?;
    Ok(banks.into_iter().map(|bank| {
        BankSimpleModel::from(bank)
    }).collect_vec())
}

#[tauri::command]
pub async fn load_bank(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32
) -> Result<Option<BankDBModel>, Error> {
    if let Some(bank) = bank_service.get_bank(id).await? {
        Ok(Some(bank))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn update_bank_recharges_count(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
    count: String
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
    timer: String
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
    loss: String
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
    loss: String
) -> Result<i32, Error> {
    let actual_loss = loss.parse::<i32>()?;
    let payload = UpdateBankPayload::new(id).with_luck_loss(actual_loss);
    bank_service.update_bank(payload).await?;
    Ok(actual_loss)
}

#[tauri::command]
pub async fn load_bank_difficulties(
    bank_service: State<'_, BanksGeneratorRepo>,
    bank_id: i32
) -> Result<Vec<BankDifficultyModel>, Error> {
    Ok(bank_service.get_difficulties(bank_id).await?.into_iter().map(|d| {
        BankDifficultyModel::from(d)
    }).collect_vec())
}

#[tauri::command]
pub async fn update_bank_difficulty_chance(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
    chance: String
) -> Result<i32, Error> {
    let actual_chance = chance.parse::<i32>()?;
    bank_service.update_difficulty(id, actual_chance).await?;
    Ok(actual_chance)
}

#[tauri::command]
pub async fn create_bank_variant(
    bank_service: State<'_, BanksGeneratorRepo>,
    bank_id: i32,
    label: String,
    difficulty: BankDifficultyType 
) -> Result<BankVariantDBModel, Error> {
    let new_variant = bank_service.create_variant(CreateVariantPayload { bank_id, label, difficulty: difficulty.into() }).await?;
    Ok(new_variant)
}

#[tauri::command]
pub async fn load_bank_variant(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32
) -> Result<Option<BankVariantDBModel>, Error> {
    if let Some(variant) = bank_service.get_variant(id).await? {
        Ok(Some(variant))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn load_bank_variants(
    bank_service: State<'_, BanksGeneratorRepo>,
    bank_id: i32
) -> Result<Vec<BankVariantDBModel>, Error> {
    let variants = bank_service.get_variants(bank_id).await?;
    Ok(variants)
}

#[tauri::command]
pub async fn update_bank_variant_label(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
    label: String
) -> Result<(), Error> {
    let payload = UpdateBankVariantPayload::new(id).with_label(label);
    bank_service.update_variant(payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_bank_variant_difficulty(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32,
    difficulty: BankDifficultyType
) -> Result<(), Error> {
    let payload = UpdateBankVariantPayload::new(id).with_difficulty(difficulty.into());
    bank_service.update_variant(payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn load_creature_slots_ids(
    bank_service: State<'_, BanksGeneratorRepo>,
    variant_id: i32
) -> Result<Vec<i32>, Error> {
    Ok(bank_service.load_creature_entries(variant_id).await?)
}

#[tauri::command]
pub async fn create_creature_slot(
    bank_service: State<'_, BanksGeneratorRepo>,
    variant_id: i32,
    slot_type: BankCreatureSlotType
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
        },
        BankCreatureSlotType::Concrete => {
            slot_data.creature_id = Some(1);
        }
    }
    let new_id = bank_service.create_creature_entry(variant_id, slot_type.into(), slot_data).await?;
    Ok(new_id)
}

#[tauri::command]
pub async fn load_creature_slot(
    bank_service: State<'_, BanksGeneratorRepo>,
    id: i32
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
    power: String
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
    grow: String
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
    town: Town
) -> Result<(), Error> {
    let payload = UpdateCreatureEntryPayload::new(slot_id).with_town(town);
    bank_service.update_creature_entry(payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_creature_slot_tier(
    bank_service: State<'_, BanksGeneratorRepo>,
    slot_id: i32,
    tier: i32
) -> Result<(), Error> {
    let payload = UpdateCreatureEntryPayload::new(slot_id).with_tier(tier);
    bank_service.update_creature_entry(payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn generate_banks_script(
    bank_service: State<'_, BanksGeneratorRepo>
) -> Result<(), Error> {
    let banks = bank_service.get_banks().await?;
    for bank in banks {
        let bank_local_type = BankType::from(bank._type.clone());
        let bank_file_name = bank_service.path.join(format!("{}.lua", bank_local_type.to_string().replace("BTD_BANK_", "").to_lowercase()));
        let mut bank_file = std::fs::File::create(bank_file_name)?;
        // loading data
        let mut bank_data_string = format!("while not {} and BTD_BANKS_DATA do\n\tsleep()\nend\n\n", bank_local_type);
        // base bank info
        bank_data_string += &format!("BTD_BANKS_DATA[{}] = {{\n", bank_local_type);
        bank_data_string += &format!(
            "\trecharges_count = {},\n\trecharge_timer = {},\n\tmorale_loss = {},\n\tluck_loss = {},\n",
            bank.recharge_count,
            bank.recharge_timer,
            bank.morale_loss,
            bank.luck_loss
        );
        // bank difficulties info 
        let bank_difficulties_data = bank_service.get_difficulties(bank.id).await?;
        bank_data_string += "\tgeneration_chances = {\n";
        for difficulty in bank_difficulties_data {
            bank_data_string += &format!("\t\t[{}] = {},\n", BankDifficultyType::from(difficulty.difficulty_type), difficulty.chance)
        }
        bank_data_string += "\t},\n";
        // bank variants info
        let bank_variants = bank_service.get_variants(bank.id).await?;
        bank_data_string += "\tvariants = {\n";
        for (variants_count, variant) in bank_variants.into_iter().enumerate() {
            bank_data_string += &format!(
                "\t\t[{}] = {{\n\t\t\tdifficulty = {},\n", 
                variants_count, 
                BankDifficultyType::from(variant.difficulty)
            );
            let creature_slots = bank_service.load_full_creature_entries(variant.id).await?;
            bank_data_string += "\t\t\tcreatures = {\n";
            for slot in creature_slots {
                bank_data_string += &format!("\t\t\t\t{{\n\t\t\t\t\ttype = {},\n", slot._type);
                if let Some(town) = slot.data.creature_town {
                    bank_data_string += &format!("\t\t\t\t\ttown = {},\n", town);
                }
                if let Some(tier) = slot.data.creature_tier {
                    bank_data_string += &format!("\t\t\t\t\ttier = {},\n", tier);
                }
                if let Some(base_power) = slot.data.base_power {
                    bank_data_string += &format!("\t\t\t\t\tbase_power = {},\n", base_power);
                }
                if let Some(power_grow) = slot.data.power_grow {
                    bank_data_string += &format!("\t\t\t\t\tpower_grow = {}, \n", power_grow);
                }
                bank_data_string.push_str("\t\t\t\t},\n");
            }
            bank_data_string.push_str("\t\t\t}\n\t\t},\n");
        }
        bank_data_string.push_str("\t}\n}");
        bank_file.write_all(bank_data_string.as_bytes())?;
    }
    Ok(())
}