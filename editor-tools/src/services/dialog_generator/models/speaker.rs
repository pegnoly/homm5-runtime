use sea_orm::FromJsonQueryResult;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub enum SpeakerType {
    NoSpeaker,
    Hero(String),
    Creature(i32),
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "speakers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name_override: Option<String>,
    pub color: String,
    pub speaker_type: SpeakerType,
    pub local_to_map: Option<i32>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
