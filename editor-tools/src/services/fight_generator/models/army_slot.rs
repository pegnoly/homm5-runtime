use super::common::{DifficultyMappedValue, AssetUpdateType, StatSelectionRule, StatParam};
use homm5_scaner::prelude::TownExtended;
use sea_orm::{FromJsonQueryResult, FromQueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use std::str;
use strum::{Display, EnumString, FromRepr};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "generated_army_slots")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub number: i32,
    pub asset_id: i32,
    pub sheet_name: String,
    pub selection_type: SelectionType,
    pub calculation_type: CalculationRule,
    pub update_type: AssetUpdateType,
    pub base_powers: DifficultyMappedValue,
    pub powers_grow: DifficultyMappedValue,
    pub towns: CreatureTowns,
    pub tiers: CreatureTiers,
    pub rules: StackGenerationRules,
    pub specific_creatures: CreatureIds,
    pub specific_count: DifficultyMappedValue,
    pub stat_rules: StackStatsRules
}

pub type ArmySlotEntity = Entity;

#[derive(Debug, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "ArmySlotEntity")]
pub struct ArmySlotId {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct CreatureTowns {
    pub towns: Vec<TownExtended>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct CreatureTiers {
    pub tiers: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct CreatureIds {
    pub ids: Vec<i32>,
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
    FromRepr,
)]
#[repr(i32)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum SelectionType {
    #[sea_orm(string_value = "MANUAL")]
    #[serde(rename = "MANUAL")]
    #[strum(serialize = "MANUAL")]
    #[default]
    Manual,
    #[sea_orm(string_value = "TIER_SLOT_BASED")]
    #[serde(rename = "TIER_SLOT_BASED")]
    #[strum(serialize = "TIER_SLOT_BASED")]
    TierSlotBased,
}

#[derive(
    Debug,
    DeriveActiveEnum,
    EnumIter,
    EnumString,
    FromRepr,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    Display,
    Default
)]
#[repr(i32)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum CalculationRule {
    #[sea_orm(string_value = "RAW")]
    #[serde(rename = "RAW")]
    #[strum(serialize = "RAW")]
    #[default]
    Raw,
    #[sea_orm(string_value = "POWER_BASED")]
    #[serde(rename = "POWER_BASED")]
    #[strum(serialize = "POWER_BASED")]
    PowerBased,
}

#[derive(
    Debug, DeriveActiveEnum, EnumIter, EnumString, PartialEq, Eq, Clone, Serialize, Deserialize, Default, FromRepr
)]
#[repr(i32)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum StackGenerationRule {
    #[sea_orm(string_value = "UPGRADE_ONLY")]
    #[serde(rename = "UPGRADE_ONLY")]
    #[strum(serialize = "UPGRADE_ONLY")]
    #[default]
    UpgradeOnly = 1,
    #[sea_orm(string_value = "GENERATABLE")]
    #[serde(rename = "GENERATABLE")]
    #[strum(serialize = "GENERATABLE")]
    Generatable = 0,
    #[sea_orm(string_value = "SHOOTER")]
    #[serde(rename = "SHOOTER")]
    #[strum(serialize = "SHOOTER")]
    Shooter = 2,
    #[sea_orm(string_value = "CASTER")]
    #[serde(rename = "CASTER")]
    #[strum(serialize = "CASTER")]
    Caster = 3,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct StackGenerationRules {
    pub rules: Vec<StackGenerationRule>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct StackStatsRules {
    pub rules: Vec<StackStatsRuleModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct StackStatsRuleModel {
    pub param: StatParam,
    pub rule: StatSelectionRule
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
