use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use super::bank_variant;

#[derive(Debug, EnumString, Display, Clone, Serialize, Deserialize, DeriveActiveEnum, EnumIter, PartialEq, Eq)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum BankType {
    // #[serde(rename = "BTD_BANK_CRYPT")]
    // #[strum(to_string = "BTD_BANK_CRYPT")]
    Crypt = 0,
    // #[serde(rename = "BTD_BANK_PYRAMID")]
    // #[strum(to_string = "BTD_BANK_PYRAMID")]
    Pyramid = 1,
    // #[serde(rename = "BTD_BANK_MAGI_VAULT")]
    // #[strum(to_string = "BTD_BANK_MAGI_VAULT")]
    MagiVault = 2,
    // #[serde(rename = "BTD_BANK_DRAGON_UTOPIA")]
    // #[strum(to_string = "BTD_BANK_DRAGON_UTOPIA")]
    DragonUtopia = 3,
    // #[serde(rename = "BTD_BANK_ELEMENTAL_STOCKPILE")]
    // #[strum(to_string = "BTD_BANK_ELEMENTAL_STOCKPILE")]
    ElementalStockpile = 4,
    // #[serde(rename = "BTD_BANK_DWARVEN_TREASURE")]
    // #[strum(to_string = "BTD_BANK_DWARVEN_TREASURE")]
    DwarvenTreasure = 5,
    // #[serde(rename = "BTD_BANK_BLOOD_TEMPLE")]
    // #[strum(to_string = "BTD_BANK_BLOOD_TEMPLE")]
    BloodTemple = 6,
    // #[serde(rename = "BTD_BANK_TREANT_THICKET")]
    // #[strum(to_string = "BTD_BANK_TREANT_THICKET")]
    TreantThicket = 7,
    // #[serde(rename = "BTD_BANK_GARGOYLE_STONEVAULT")]
    // #[strum(to_string = "BTD_BANK_GARGOYLE_STONEVAULT")]
    GargoyleStonevault = 8,
    // #[serde(rename = "BTD_BANK_SUNKEN_TEMPLE")]
    // #[strum(to_string = "BTD_BANK_SUNKEN_TEMPLE")]
    SunkenTemple = 9
}

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "banks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub _type: BankType,
    pub name: String,
    pub recharge_count: i32,
    pub recharge_timer: i32,
    pub luck_loss: i32,
    pub morale_loss: i32
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    BankVariant
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::BankVariant => Entity::has_many(bank_variant::Entity).into()
        }
    }
}

impl Related<bank_variant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BankVariant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}