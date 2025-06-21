use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "speakers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub dialog_id: i32,
    pub step: i32,
    pub label: String,
    pub speaker_id: i32,
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}