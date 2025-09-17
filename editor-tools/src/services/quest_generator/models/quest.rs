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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
