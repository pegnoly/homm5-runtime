use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

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
#[sea_orm(table_name = "bank_difficulties")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub bank_id: i32,
    pub difficulty_type: BankDifficultyType,
    pub chance: i32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}