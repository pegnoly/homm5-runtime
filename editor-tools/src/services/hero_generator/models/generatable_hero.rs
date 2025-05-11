use std::collections::HashMap;
use homm5_scaner::prelude::ArtifactSlotType;
use sea_orm::{prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "generatable_heroes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub path_to_generate: String,
    pub table_name: String,
    pub optional_artifacts: OptionalArtifactsModel
}

#[derive(Debug, DeriveActiveEnum, EnumIter, EnumString, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum DifficultyType {
    #[sea_orm(string_value = "DIFFICULTY_EASY")]
    #[serde(rename = "DIFFICULTY_EASY")]
    #[strum(serialize = "DIFFICULTY_EASY")]
    Easy,
    #[sea_orm(string_value = "DIFFICULTY_MEDIUM")]
    #[serde(rename = "DIFFICULTY_MEDIUM")]
    #[strum(serialize = "DIFFICULTY_MEDIUM")]
    Medium,
    #[sea_orm(string_value = "DIFFICULTY_HARD")]
    #[serde(rename = "DIFFICULTY_HARD")]
    #[strum(serialize = "DIFFICULTY_HARD")]
    Hard, 
    #[sea_orm(string_value = "DIFFICULTY_HEROIC")]
    #[serde(rename = "DIFFICULTY_HEROIC")]
    #[strum(serialize = "DIFFICULTY_HEROIC")]
    Heroic
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct StaticDifficultyData {
    pub values: HashMap<DifficultyType, i32>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DynamicDifficulty {
    pub base_value: i32,
    pub grow_per_week: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DynamicDifficultyData {
    pub values: HashMap<DifficultyType, DynamicDifficulty>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ArtifactsGenerationCostType {
    NotSelected,
    Static(StaticDifficultyData),
    Dynamic(DynamicDifficultyData)
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct OptionalArtifactsModel {
    pub artifacts: HashMap<ArtifactSlotType, Vec<i32>>,
    pub cost_type: ArtifactsGenerationCostType
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}