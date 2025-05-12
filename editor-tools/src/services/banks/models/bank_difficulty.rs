use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug,
    EnumString,
    Display,
    Clone,
    Serialize,
    Deserialize,
    DeriveActiveEnum,
    EnumIter,
    PartialEq,
    Eq,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum BankDifficultyType {
    #[sea_orm(string_value = "BANK_DIFFICULTY_EASY")]
    #[serde(rename = "BANK_DIFFICULTY_EASY")]
    #[strum(serialize = "BANK_DIFFICULTY_EASY")]
    Easy,
    #[sea_orm(string_value = "BANK_DIFFICULTY_MEDIUM")]
    #[serde(rename = "BANK_DIFFICULTY_MEDIUM")]
    #[strum(serialize = "BANK_DIFFICULTY_MEDIUM")]
    Medium,
    #[sea_orm(string_value = "BANK_DIFFICULTY_HARD")]
    #[serde(rename = "BANK_DIFFICULTY_HARD")]
    #[strum(serialize = "BANK_DIFFICULTY_HARD")]
    Hard,
    #[sea_orm(string_value = "BANK_DIFFICULTY_CRITICAL")]
    #[serde(rename = "BANK_DIFFICULTY_CRITICAL")]
    #[strum(serialize = "BANK_DIFFICULTY_CRITICAL")]
    Critical,
    #[sea_orm(string_value = "BANK_DIFFICULTY_BOSS")]
    #[serde(rename = "BANK_DIFFICULTY_BOSS")]
    #[strum(serialize = "BANK_DIFFICULTY_BOSS")]
    Boss,
}

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "bank_difficulties")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub bank_id: i32,
    pub difficulty_type: BankDifficultyType,
    pub chance: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
