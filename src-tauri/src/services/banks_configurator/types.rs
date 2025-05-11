use derive_more::derive::Debug;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use editor_tools::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct BankSimpleModel {
    pub id: i32,
    pub name: String
}

impl From<BankDBModel> for BankSimpleModel {
    fn from(value: BankDBModel) -> Self {
        BankSimpleModel { id: value.id, name: value.name.clone() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankDifficultyModel {
    pub id: i32,
    pub difficulty: BankDifficultyType,
    pub chance: i32
}

impl From<BankDifficultyDBModel> for BankDifficultyModel {
    fn from(value: BankDifficultyDBModel) -> Self {
        BankDifficultyModel {
            id: value.id,
            difficulty: BankDifficultyType::from(value.difficulty_type),
            chance: value.chance
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankVariantModel {
    pub id: i32,
    pub label: String,
    //pub chance: i32,
    pub difficulty: BankDifficultyType
}

impl From<BankVariantDBModel> for BankVariantModel {
    fn from(value: BankVariantDBModel) -> Self {
        BankVariantModel {
            id: value.id,
            //chance: value.chance,
            label: value.label,
            difficulty: BankDifficultyType::from(value.difficulty)
        }
    }
}