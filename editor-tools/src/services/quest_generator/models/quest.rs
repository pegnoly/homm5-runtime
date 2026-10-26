use sea_orm::FromJsonQueryResult;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "quests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub mission_id: i32,
    pub directory: String,
    pub name: String,
    pub script_name: String,
    pub desc: String,
    pub is_active: bool,
    pub is_secondary: bool,
    pub update_sources: QuestUpdateSources
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum QuestUpdateSourceVariableType {
    String,
    Number,
    Bool,
    #[default]
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestUpdateSourceVariable {
    pub name: String,
    #[serde(default, rename = "type")]
    pub variable_type: QuestUpdateSourceVariableType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestUpdateSource {
    pub source: String,
    pub variables: Vec<QuestUpdateSourceVariable>,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromJsonQueryResult)]
pub struct QuestUpdateSources {
    pub sources: Vec<QuestUpdateSource>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
