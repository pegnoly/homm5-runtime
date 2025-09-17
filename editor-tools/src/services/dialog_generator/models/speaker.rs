use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

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
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum SpeakerType {
    #[sea_orm(string_value = "SPEAKER_TYPE_NO_SPEAKER")]
    #[serde(rename = "SPEAKER_TYPE_NO_SPEAKER")]
    #[strum(to_string = "SPEAKER_TYPE_NO_SPEAKER")]
    NoSpeaker,
    #[sea_orm(string_value = "SPEAKER_TYPE_HERO")]
    #[serde(rename = "SPEAKER_TYPE_HERO")]
    #[strum(to_string = "SPEAKER_TYPE_HERO")]
    Hero,
    #[sea_orm(string_value = "SPEAKER_TYPE_CREATURE")]
    #[serde(rename = "SPEAKER_TYPE_CREATURE")]
    #[strum(to_string = "SPEAKER_TYPE_CREATURE")]
    Creature,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "speakers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub script_name: String,
    pub color: String,
    pub speaker_type: SpeakerType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
