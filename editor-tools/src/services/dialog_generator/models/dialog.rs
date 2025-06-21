use sea_orm::{prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dialogs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub mission_id: i32,
    pub name: String,
    pub script_name: String,
    pub directory: String,
    pub speakers_ids: SpeakersIds,
    pub labels: Labels,
    pub was_generated: bool
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]

pub struct SpeakersIds {
    pub ids: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]

pub struct Labels {
    pub labels: Vec<String>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}