use std::collections::HashMap;

use sea_orm::{FromJsonQueryResult, Iterable, prelude::*};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug,
    DeriveActiveEnum,
    EnumIter,
    EnumString,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    Hash,
    Display
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum DifficultyType {
    #[sea_orm(string_value = "DIFFICULTY_EASY")]
    #[serde(rename = "DIFFICULTY_EASY")]
    #[strum(serialize = "DIFFICULTY_EASY")]
    Easy,
    #[sea_orm(string_value = "DIFFICULTY_NORMAL")]
    #[serde(rename = "DIFFICULTY_NORMAL")]
    #[strum(serialize = "DIFFICULTY_NORMAL")]
    Medium,
    #[sea_orm(string_value = "DIFFICULTY_HARD")]
    #[serde(rename = "DIFFICULTY_HARD")]
    #[strum(serialize = "DIFFICULTY_HARD")]
    Hard,
    #[sea_orm(string_value = "DIFFICULTY_HEROIC")]
    #[serde(rename = "DIFFICULTY_HEROIC")]
    #[strum(serialize = "DIFFICULTY_HEROIC")]
    Heroic,
}

#[derive(
    Debug, DeriveActiveEnum, EnumIter, EnumString, PartialEq, Eq, Clone, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AssetGenerationType {
    #[sea_orm(string_value = "GENERATION_TYPE_STATIC")]
    #[serde(rename = "GENERATION_TYPE_STATIC")]
    #[strum(serialize = "GENERATION_TYPE_STATIC")]
    Static,
    #[sea_orm(string_value = "GENERATION_TYPE_DYNAMIC")]
    #[serde(rename = "GENERATION_TYPE_DYNAMIC")]
    #[strum(serialize = "GENERATION_TYPE_DYNAMIC")]
    Dynamic,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct DifficultyMappedValue {
    pub data: HashMap<DifficultyType, i32>,
}

impl Default for DifficultyMappedValue {
    fn default() -> Self {
        DifficultyMappedValue {
            data: HashMap::from_iter(DifficultyType::iter().map(|d| (d, 0))),
        }
    }
}
