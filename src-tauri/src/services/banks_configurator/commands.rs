use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tauri::State;
use editor_tools::services::banks::{models::{bank, bank_creature_entry::CreatureSlotData}, service::{payloads::CreateVariantPayload, BanksService}};

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
pub async fn create_variant(
    bank_service: State<'_, BanksService>,
    bank_id: i32,
    chance: i32,
    difficulty: BankDifficultyType 
) -> Result<BankVariantModel, Error> {
    let new_variant = bank_service.create_variant(CreateVariantPayload { bank_id, chance, difficulty: difficulty.into() }).await?;
    Ok(BankVariantModel::from(new_variant))
}

#[tauri::command]
pub async fn load_variant(
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
    let mut slot_data = CreatureSlotData {
        base_power: Some(0),
        power_grow: Some(0),
        ..Default::default()
    };
    match slot_type {
        CreatureSlotType::Tier => {
            slot_data.creature_tier = Some(1);
            slot_data.creature_town = Some(1);
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
) -> Result<Option<CreatureSlotData>, Error> {
    if let Some(slot_data) = bank_service.load_creature_entry(id).await? {
        Ok(Some(slot_data.data))
    } else {
        Ok(None)
    }
}