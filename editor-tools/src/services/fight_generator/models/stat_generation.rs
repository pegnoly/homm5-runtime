#![allow(clippy::enum_variant_names)]

use sea_orm::{FromJsonQueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use std::str;
use strum::EnumString;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "army_generation_stat_elements")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub stack_id: i32,
    pub rule: ArmyGenerationStatRule,
    pub priority: i32,
    pub stats: ArmyGenerationStats,
}

#[derive(
    Debug, DeriveActiveEnum, EnumIter, EnumString, PartialEq, Eq, Clone, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ArmyGenerationStatParam {
    #[sea_orm(string_value = "GENERATION_STAT_INITIATIVE")]
    #[serde(rename = "GENERATION_STAT_INITIATIVE")]
    #[strum(serialize = "GENERATION_STAT_INITIATIVE")]
    Initiative,
    #[sea_orm(string_value = "GENERATION_STAT_SPEED")]
    #[serde(rename = "GENERATION_STAT_SPEED")]
    #[strum(serialize = "GENERATION_STAT_SPEED")]
    Speed,
    #[sea_orm(string_value = "GENERATION_STAT_HITPOINTS")]
    #[serde(rename = "GENERATION_STAT_HITPOINTS")]
    #[strum(serialize = "GENERATION_STAT_HITPOINTS")]
    Hitpoints,
    #[sea_orm(string_value = "GENERATION_STAT_ATTACK")]
    #[serde(rename = "GENERATION_STAT_ATTACK")]
    #[strum(serialize = "GENERATION_STAT_ATTACK")]
    Attack,
    #[sea_orm(string_value = "GENERATION_STAT_DEFENCE")]
    #[serde(rename = "GENERATION_STAT_DEFENCE")]
    #[strum(serialize = "GENERATION_STAT_DEFENCE")]
    Defence,
}

#[derive(
    Debug, DeriveActiveEnum, EnumIter, EnumString, PartialEq, Eq, Clone, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ArmyGenerationStatRule {
    #[sea_orm(string_value = "GENERATION_STAT_RULE_MAXBY")]
    #[serde(rename = "GENERATION_STAT_RULE_MAXBY")]
    #[strum(serialize = "GENERATION_STAT_RULE_MAXBY")]
    MaxBy,
    #[sea_orm(string_value = "GENERATION_STAT_RULE_MINBY")]
    #[serde(rename = "GENERATION_STAT_RULE_MINBY")]
    #[strum(serialize = "GENERATION_STAT_RULE_MINBY")]
    MinBy,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct ArmyGenerationStats {
    pub values: Vec<ArmyGenerationStatParam>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
