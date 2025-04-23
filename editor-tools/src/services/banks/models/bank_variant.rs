use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use sea_orm::prelude::*;

use super::bank;

#[derive(Debug, EnumString, Display, Clone, Serialize, Deserialize, DeriveActiveEnum, EnumIter, PartialEq, Eq)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum BankDifficultyType {
    Easy = 0,
    Medium = 1,
    Hard = 2,
    Critical = 3,
    Boss = 4
}

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "bank_variants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub bank_id: i32,
    pub chance: i32,
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