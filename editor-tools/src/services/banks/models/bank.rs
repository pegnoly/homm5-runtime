use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use super::bank_variant;

#[derive(
    Debug,
    EnumString,
    Display,
    Clone,
    Serialize,
    Deserialize,
    DeriveActiveEnum,
    EnumIter,
    PartialEq,
    Eq,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum BankType {
    #[sea_orm(string_value = "BTD_BANK_CRYPT")]
    #[serde(rename = "BTD_BANK_CRYPT")]
    #[strum(serialize = "BTD_BANK_CRYPT")]
    Crypt,
    #[sea_orm(string_value = "BTD_BANK_PYRAMID")]
    #[serde(rename = "BTD_BANK_PYRAMID")]
    #[strum(serialize = "BTD_BANK_PYRAMID")]
    Pyramid,
    #[sea_orm(string_value = "BTD_BANK_MAGI_VAULT")]
    #[serde(rename = "BTD_BANK_MAGI_VAULT")]
    #[strum(serialize = "BTD_BANK_MAGI_VAULT")]
    MagiVault,
    #[sea_orm(string_value = "BTD_BANK_DRAGON_UTOPIA")]
    #[serde(rename = "BTD_BANK_DRAGON_UTOPIA")]
    #[strum(serialize = "BTD_BANK_DRAGON_UTOPIA")]
    DragonUtopia,
    #[sea_orm(string_value = "BTD_BANK_ELEMENTAL_STOCKPILE")]
    #[serde(rename = "BTD_BANK_ELEMENTAL_STOCKPILE")]
    #[strum(serialize = "BTD_BANK_ELEMENTAL_STOCKPILE")]
    ElementalStockpile,
    #[sea_orm(string_value = "BTD_BANK_DWARVEN_TREASURE")]
    #[serde(rename = "BTD_BANK_DWARVEN_TREASURE")]
    #[strum(serialize = "BTD_BANK_DWARVEN_TREASURE")]
    DwarvenTreasure,
    #[sea_orm(string_value = "BTD_BANK_BLOOD_TEMPLE")]
    #[serde(rename = "BTD_BANK_BLOOD_TEMPLE")]
    #[strum(serialize = "BTD_BANK_BLOOD_TEMPLE")]
    BloodTemple,
    #[sea_orm(string_value = "BTD_BANK_TREANT_THICKET")]
    #[serde(rename = "BTD_BANK_TREANT_THICKET")]
    #[strum(serialize = "BTD_BANK_TREANT_THICKET")]
    TreantThicket,
    #[sea_orm(string_value = "BTD_BANK_GARGOYLE_STONEVAULT")]
    #[serde(rename = "BTD_BANK_GARGOYLE_STONEVAULT")]
    #[strum(serialize = "BTD_BANK_GARGOYLE_STONEVAULT")]
    GargoyleStonevault,
    #[sea_orm(string_value = "BTD_BANK_SUNKEN_TEMPLE")]
    #[serde(rename = "BTD_BANK_SUNKEN_TEMPLE")]
    #[strum(serialize = "BTD_BANK_SUNKEN_TEMPLE")]
    SunkenTemple,
}

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "banks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "type")]
    pub _type: BankType,
    pub name: String,
    pub recharge_count: i32,
    pub recharge_timer: i32,
    pub luck_loss: i32,
    pub morale_loss: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    BankVariant,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::BankVariant => Entity::has_many(bank_variant::Entity).into(),
        }
    }
}

impl Related<bank_variant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BankVariant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
