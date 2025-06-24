use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "quest_progresses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub quest_id: i32,
    pub number: i32,
    pub text: String,
    pub concatenate: bool
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}