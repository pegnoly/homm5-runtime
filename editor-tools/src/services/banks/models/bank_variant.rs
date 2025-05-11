use serde::{Deserialize, Serialize};
use sea_orm::prelude::*;

use super::{bank, bank_difficulty::BankDifficultyType};

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "bank_variants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub bank_id: i32,
    pub label: String,
    pub difficulty: BankDifficultyType
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Bank
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Bank => Entity::belongs_to(bank::Entity)
                .from(Column::BankId)
                .to(bank::Column::Id) 
                .into()
        }
    }
}

impl Related<bank::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bank.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}