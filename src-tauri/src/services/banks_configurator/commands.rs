use itertools::Itertools;
use tauri::State;
use editor_tools::services::banks::{models::{self, bank_creature_entry::CreatureTownType}, service::{payloads::{CreateVariantPayload, UpdateBankPayload, UpdateBankVariantPayload, UpdateCreatureEntryPayload}, BanksService}};
use crate::{error::Error, services::banks_configurator::types::{BankModel, BankSimpleModel}};
use super::types::{BankDifficultyType, BankVariantModel, CreatureSlotType};

#[tauri::command]
pub async fn get_all_banks(
    bank_service: State<'_, BanksService>
) -> Result<Vec<BankSimpleModel>, Error> {
    let banks = bank_service.get_banks().await?;
    Ok(banks.into_iter().map(|bank| {
        BankSimpleModel::from(bank)
    }).collect_vec())
}

#[tauri::command]
pub async fn load_bank(
    bank_service: State<'_, BanksService>,
    id: i32
) -> Result<Option<BankModel>, Error> {
    if let Some(bank) = bank_service.get_bank(id).await? {
        Ok(Some(BankModel::from(bank)))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn update_bank_recharges_count(
    bank_service: State<'_, BanksService>,
    id: i32,
    count: String
) -> Result<i32, Error> {
    let actual_count = i32::from_str_radix(&count, 10)?;
    let payload = UpdateBankPayload::new(id).with_recharge_count(actual_count);
    bank_service.update_bank(payload).await?;
    Ok(actual_count)
}

#[tauri::command]
pub async fn update_bank_recharge_timer(
    bank_service: State<'_, BanksService>,
    id: i32,
    timer: String
) -> Result<i32, Error> {
    let actual_timer = i32::from_str_radix(&timer, 10)?;
    let payload = UpdateBankPayload::new(id).with_recharge_timer(actual_timer);
    bank_service.update_bank(payload).await?;
    Ok(actual_timer)
}

#[tauri::command]
pub async fn update_bank_morale_loss(
    bank_service: State<'_, BanksService>,
    id: i32,
    loss: String
) -> Result<i32, Error> {
    let actual_loss = i32::from_str_radix(&loss, 10)?;
    let payload = UpdateBankPayload::new(id).with_morale_loss(actual_loss);
    bank_service.update_bank(payload).await?;
    Ok(actual_loss)
}

#[tauri::command]
pub async fn update_bank_luck_loss(
    bank_service: State<'_, BanksService>,
    id: i32,
    loss: String
) -> Result<i32, Error> {
    let actual_loss = i32::from_str_radix(&loss, 10)?;
    let payload = UpdateBankPayload::new(id).with_luck_loss(actual_loss);
    bank_service.update_bank(payload).await?;
    Ok(actual_loss)
}


#[tauri::command]
pub async fn create_bank_variant(
    bank_service: State<'_, BanksService>,
    bank_id: i32,
    chance: i32,
    difficulty: BankDifficultyType 
) -> Result<BankVariantModel, Error> {
    let new_variant = bank_service.create_variant(CreateVariantPayload { bank_id, chance, difficulty: difficulty.into() }).await?;
    Ok(BankVariantModel::from(new_variant))
}

#[tauri::command]
pub async fn load_bank_variant(
    bank_service: State<'_, BanksService>,
    id: i32
) -> Result<Option<BankVariantModel>, Error> {
    if let Some(variant) = bank_service.get_variant(id).await? {
        Ok(Some(BankVariantModel::from(variant)))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn load_bank_variants(
    bank_service: State<'_, BanksService>,
    bank_id: i32
) -> Result<Vec<BankVariantModel>, Error> {
    let variants = bank_service.get_variants(bank_id).await?;
    Ok(variants.into_iter().map(|variant| {
        BankVariantModel::from(variant)
    }).collect_vec())
}

#[tauri::command]
pub async fn update_bank_variant_chance(
    bank_service: State<'_, BanksService>,
    variant_id: i32,
    chance: String
) -> Result<i32, Error> {
    let actual_chance = i32::from_str_radix(&chance, 10)?;
    let payload = UpdateBankVariantPayload::new(variant_id).with_chance(actual_chance);
    bank_service.update_variant(payload).await?;
    Ok(actual_chance)
}

#[tauri::command]
pub async fn load_creature_slots_ids(
    bank_service: State<'_, BanksService>,
    variant_id: i32
) -> Result<Vec<i32>, Error> {
    Ok(bank_service.load_creature_entries(variant_id).await?)
}

#[tauri::command]
pub async fn create_creature_slot(
    bank_service: State<'_, BanksService>,
    variant_id: i32,
    slot_type: CreatureSlotType
) -> Result<i32, Error> {
    let mut slot_data = models::bank_creature_entry::CreatureSlotData {
        base_power: Some(0),
        power_grow: Some(0),
        ..Default::default()
    };
    match slot_type {
        CreatureSlotType::Tier => {
            slot_data.creature_tier = Some(1);
            slot_data.creature_town = Some(models::bank_creature_entry::CreatureTownType::TownNoType);
        },
        CreatureSlotType::Concrete => {
            slot_data.creature_id = Some(1);
        }
    }
    let new_id = bank_service.create_creature_entry(variant_id, slot_type.into(), slot_data).await?;
    Ok(new_id)
}

#[tauri::command]
pub async fn load_creature_slot(
    bank_service: State<'_, BanksService>,
    id: i32
) -> Result<Option<models::bank_creature_entry::CreatureSlotData>, Error> {
    if let Some(slot_data) = bank_service.load_creature_entry(id).await? {
        Ok(Some(slot_data.data))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn update_creature_slot_base_power(
    bank_service: State<'_, BanksService>,
    slot_id: i32,
    power: String
) -> Result<i32, Error> {
    let actual_power = i32::from_str_radix(&power, 10)?;
    let payload = UpdateCreatureEntryPayload::new(slot_id).with_base_power(actual_power);
    bank_service.update_creature_entry(payload).await?;
    Ok(actual_power)
}

#[tauri::command]
pub async fn update_creature_slot_power_grow(
    bank_service: State<'_, BanksService>,
    slot_id: i32,
    grow: String
) -> Result<i32, Error> {
    let actual_grow = i32::from_str_radix(&grow, 10)?;
    let payload = UpdateCreatureEntryPayload::new(slot_id).with_power_grow(actual_grow);
    bank_service.update_creature_entry(payload).await?;
    Ok(actual_grow)
}

#[tauri::command]
pub async fn update_creature_slot_town(
    bank_service: State<'_, BanksService>,
    slot_id: i32,
    town: CreatureTownType
) -> Result<(), Error> {
    let payload = UpdateCreatureEntryPayload::new(slot_id).with_town(town);
    bank_service.update_creature_entry(payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_creature_slot_tier(
    bank_service: State<'_, BanksService>,
    slot_id: i32,
    tier: i32
) -> Result<(), Error> {
    let payload = UpdateCreatureEntryPayload::new(slot_id).with_tier(tier);
    bank_service.update_creature_entry(payload).await?;
    Ok(())
}