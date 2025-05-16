use homm5_scaner::prelude::Town;
use sea_orm::{prelude::*, FromJsonQueryResult, FromQueryResult};
use serde::{Deserialize, Serialize};
use std::str;
use strum::EnumString;

use super::common::{AssetGenerationType, DifficultyMappedValue};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "hero_generated_army_slots")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub asset_id: i32,
    pub type_generation_mode: ArmySlotStackUnitGenerationMode,
    pub count_generation_mode: ArmySlotStackCountGenerationMode,
    pub power_based_generation_type: AssetGenerationType,
    pub base_powers: DifficultyMappedValue,
    pub powers_grow: DifficultyMappedValue,
    pub town: Town,
    pub tier: i32,
    pub generation_rule: ArmySlotGenerationRule,
    pub concrete_creature: i32,
    pub concrete_count: DifficultyMappedValue
}

pub type ArmySlotEntity = Entity;

#[derive(Debug, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "ArmySlotEntity")]
pub struct ArmySlotId {
    pub id: i32
}

#[derive(Debug, DeriveActiveEnum, EnumIter, EnumString, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ArmySlotStackUnitGenerationMode {
    #[sea_orm(string_value = "UNIT_TYPE_GENERATION_MODE_CONCRETE")]
    #[serde(rename = "UNIT_TYPE_GENERATION_MODE_CONCRETE")]
    #[strum(serialize = "UNIT_TYPE_GENERATION_MODE_CONCRETE")]
    ConcreteUnit,
    #[sea_orm(string_value = "UNIT_TYPE_GENERATION_MODE_TIER_SLOT_BASED")]
    #[serde(rename = "UNIT_TYPE_GENERATION_MODE_TIER_SLOT_BASED")]
    #[strum(serialize = "UNIT_TYPE_GENERATION_MODE_TIER_SLOT_BASED")]
    TierSlotBased
}

#[derive(Debug, DeriveActiveEnum, EnumIter, EnumString, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ArmySlotStackCountGenerationMode {
    #[sea_orm(string_value = "UNIT_COUNT_GENERATION_MODE_RAW")]
    #[serde(rename = "UNIT_COUNT_GENERATION_MODE_RAW")]
    #[strum(serialize = "UNIT_COUNT_GENERATION_MODE_RAW")]
    Raw,
    #[sea_orm(string_value = "UNIT_COUNT_GENERATION_MODE_POWER_BASED")]
    #[serde(rename = "UNIT_COUNT_GENERATION_MODE_POWER_BASED")]
    #[strum(serialize = "UNIT_COUNT_GENERATION_MODE_POWER_BASED")]
    PowerBased
}

#[derive(
    Debug, DeriveActiveEnum, EnumIter, EnumString, PartialEq, Eq, Clone, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ArmyGenerationRuleParam {
    #[sea_orm(string_value = "GENERATION_RULE_UPGRADE_ONLY")]
    #[serde(rename = "GENERATION_RULE_UPGRADE_ONLY")]
    #[strum(serialize = "GENERATION_RULE_UPGRADE_ONLY")]
    UpgradeOnly,
    #[sea_orm(string_value = "GENERATION_RULE_GENERATABLE")]
    #[serde(rename = "GENERATION_RULE_GENERATABLE")]
    #[strum(serialize = "GENERATION_RULE_GENERATABLE")]
    Generatable,
    #[sea_orm(string_value = "GENERATION_RULE_SHOOTER")]
    #[serde(rename = "GENERATION_RULE_SHOOTER")]
    #[strum(serialize = "GENERATION_RULE_SHOOTER")]
    Shooter,
    #[sea_orm(string_value = "GENERATION_RULE_CASTER")]
    #[serde(rename = "GENERATION_RULE_CASTER")]
    #[strum(serialize = "GENERATION_RULE_CASTER")]
    Caster,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct ArmySlotGenerationRule {
    pub params: Vec<ArmyGenerationRuleParam>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}