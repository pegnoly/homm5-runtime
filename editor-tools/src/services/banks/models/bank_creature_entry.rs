use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use sea_orm::{prelude::*, FromJsonQueryResult, FromQueryResult};

#[derive(Debug, EnumString, Display, Clone, Serialize, Deserialize, DeriveActiveEnum, EnumIter, PartialEq, Eq)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum BankCreatureSlotType {
    Tier = 0,
    Concrete = 1
}

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "bank_creature_entries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub variant_id: i32,
    #[sea_orm(column_name = "type")]
    pub _type: BankCreatureSlotType,
    pub data: CreatureSlotData
}

#[derive(Debug, Clone, Serialize, Deserialize, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct CreatureSlotData {
    pub base_power: Option<i32>,
    pub power_grow: Option<i32>,
    pub creature_town: Option<i32>,
    pub creature_tier: Option<i32>,
    pub creature_id: Option<i32>,
    pub creature_count: Option<i32>
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct CreatureEntryId {
    pub id: i32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}