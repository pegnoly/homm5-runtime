#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;
use sea_orm::{FromJsonQueryResult, Iterable, prelude::*};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, FromRepr};

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
    Display,
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
    Debug,
    DeriveActiveEnum,
    EnumIter,
    EnumString,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    Display,
    Default,
    FromRepr
)]
#[repr(i32)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AssetUpdateType {
    #[sea_orm(string_value = "STATIC")]
    #[serde(rename = "STATIC")]
    #[strum(serialize = "STATIC")]
    #[default]
    Static,
    #[sea_orm(string_value = "DYNAMIC")]
    #[serde(rename = "DYNAMIC")]
    #[strum(serialize = "DYNAMIC")]
    Dynamic,
}

#[derive(
    Debug, DeriveActiveEnum, EnumIter, EnumString, PartialEq, Eq, Clone, Serialize, Deserialize, Default
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum StatParam {
    #[sea_orm(string_value = "INITIATIVE")]
    #[serde(rename = "INITIATIVE")]
    #[strum(serialize = "INITIATIVE")]
    #[default]
    Initiative,
    #[sea_orm(string_value = "SPEED")]
    #[serde(rename = "SPEED")]
    #[strum(serialize = "SPEED")]
    Speed,
    #[sea_orm(string_value = "HITPOINTS")]
    #[serde(rename = "HITPOINTS")]
    #[strum(serialize = "HITPOINTS")]
    Hitpoints,
    #[sea_orm(string_value = "ATTACK")]
    #[serde(rename = "ATTACK")]
    #[strum(serialize = "ATTACK")]
    Attack,
    #[sea_orm(string_value = "DEFENCE")]
    #[serde(rename = "DEFENCE")]
    #[strum(serialize = "DEFENCE")]
    Defence,
}

#[derive(
    Debug, DeriveActiveEnum, EnumIter, EnumString, PartialEq, Eq, Clone, Serialize, Deserialize, Default
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum StatSelectionRule {
    #[sea_orm(string_value = "MAXBY")]
    #[serde(rename = "MAXBY")]
    #[strum(serialize = "MAXBY")]
    #[default]
    MaxBy,
    #[sea_orm(string_value = "MINBY")]
    #[serde(rename = "MINBY")]
    #[strum(serialize = "MINBY")]
    MinBy,
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
