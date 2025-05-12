use homm5_scaner::prelude::Town;
use sea_orm::{FromJsonQueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use std::str;
use strum::EnumString;

use super::common::{AssetGenerationType, DifficultyMappedValue};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "hero_generated_army_slots")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub asset_id: i32,
    pub generation_type: AssetGenerationType,
    pub base_powers: DifficultyMappedValue,
    pub powers_grow: Option<DifficultyMappedValue>,
    pub town: Town,
    pub tier: i32,
    pub generation_rule: ArmySlotGenerationRule,
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
