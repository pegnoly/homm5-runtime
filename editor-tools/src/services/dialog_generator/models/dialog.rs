use sea_orm::{FromJsonQueryResult, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dialogs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub mission_id: i32,
    pub name: String,
    pub script_name: String,
    pub directory: String,
    pub speakers_ids: SpeakersIds,
    pub labels: Labels
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]

pub struct SpeakersIds {
    pub ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]

pub struct Labels {
    pub labels: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
