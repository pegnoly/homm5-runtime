use sea_orm::FromJsonQueryResult;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dialog_variants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub dialog_id: Uuid,
    pub step: i32,
    pub label: String,
    pub speaker_ids: VariantSpeakerIds,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct VariantSpeakerIds {
    pub ids: Vec<Uuid>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
