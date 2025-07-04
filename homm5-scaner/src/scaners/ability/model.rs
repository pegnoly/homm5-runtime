use homm5_types::ability::AbilityShared;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "abilities")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub game_id: String,
    pub name: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<AbilityShared> for Model {
    fn from(value: AbilityShared) -> Self {
        Model { 
            id: Default::default(), 
            game_id: value.ID, 
            name: Default::default() 
        }
    }
}