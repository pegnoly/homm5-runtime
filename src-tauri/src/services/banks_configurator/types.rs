use derive_more::derive::Debug;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use editor_tools::services::banks::models::{self, bank_difficulty, bank_variant};

#[derive(Debug, EnumString, Display, Clone, Serialize, Deserialize)]
pub enum BankType {
    #[serde(rename = "BTD_BANK_CRYPT")]
    #[strum(to_string = "BTD_BANK_CRYPT")]
    Crypt,
    #[serde(rename = "BTD_BANK_PYRAMID")]
    #[strum(to_string = "BTD_BANK_PYRAMID")]
    Pyramid,
    #[serde(rename = "BTD_BANK_MAGI_VAULT")]
    #[strum(to_string = "BTD_BANK_MAGI_VAULT")]
    MagiVault,
    #[serde(rename = "BTD_BANK_DRAGON_UTOPIA")]
    #[strum(to_string = "BTD_BANK_DRAGON_UTOPIA")]
    DragonUtopia,
    #[serde(rename = "BTD_BANK_ELEMENTAL_STOCKPILE")]
    #[strum(to_string = "BTD_BANK_ELEMENTAL_STOCKPILE")]
    ElementalStockpile,
    #[serde(rename = "BTD_BANK_DWARVEN_TREASURE")]
    #[strum(to_string = "BTD_BANK_DWARVEN_TREASURE")]
    DwarvenTreasure,
    #[serde(rename = "BTD_BANK_BLOOD_TEMPLE")]
    #[strum(to_string = "BTD_BANK_BLOOD_TEMPLE")]
    BloodTemple,
    #[serde(rename = "BTD_BANK_TREANT_THICKET")]
    #[strum(to_string = "BTD_BANK_TREANT_THICKET")]
    TreantThicket,
    #[serde(rename = "BTD_BANK_GARGOYLE_STONEVAULT")]
    #[strum(to_string = "BTD_BANK_GARGOYLE_STONEVAULT")]
    GargoyleStonevault,
    #[serde(rename = "BTD_BANK_SUNKEN_TEMPLE")]
    #[strum(to_string = "BTD_BANK_SUNKEN_TEMPLE")]
    SunkenTemple
}

impl From<models::bank::BankType> for BankType {
    fn from(value: models::bank::BankType) -> Self {
        match value {
            models::bank::BankType::Crypt => BankType::Crypt,
            models::bank::BankType::BloodTemple => BankType::BloodTemple,
            models::bank::BankType::DragonUtopia => BankType::DragonUtopia,
            models::bank::BankType::DwarvenTreasure => BankType::DwarvenTreasure,
            models::bank::BankType::ElementalStockpile => BankType::ElementalStockpile,
            models::bank::BankType::GargoyleStonevault => BankType::GargoyleStonevault,
            models::bank::BankType::MagiVault => BankType::MagiVault,
            models::bank::BankType::Pyramid => BankType::Pyramid,
            models::bank::BankType::SunkenTemple => BankType::SunkenTemple,
            models::bank::BankType::TreantThicket => BankType::TreantThicket
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankSimpleModel {
    pub id: i32,
    pub name: String
}

impl From<models::bank::Model> for BankSimpleModel {
    fn from(value: models::bank::Model) -> Self {
        BankSimpleModel { id: value.id, name: value.name.clone() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankModel {
    pub id: i32,
    #[serde(rename = "type")]
    pub _type: BankType,
    pub name: String,
    pub recharge_count: i32,
    pub recharge_timer: i32,
    pub luck_loss: i32,
    pub morale_loss: i32
}

impl From<models::bank::Model> for BankModel {
    fn from(value: models::bank::Model) -> Self {
        BankModel {
            id: value.id,
            _type: BankType::from(value._type),
            name: value.name,
            recharge_count: value.recharge_count,
            recharge_timer: value.recharge_timer,
            luck_loss: value.luck_loss,
            morale_loss: value.morale_loss
        }
    }
}

#[derive(Debug, EnumString, Display, Serialize, Deserialize)]
pub enum BankDifficultyType {
    #[serde(rename = "BANK_DIFFICULTY_EASY")]
    #[strum(to_string = "BANK_DIFFICULTY_EASY")]
    Easy,
    #[serde(rename = "BANK_DIFFICULTY_MEDIUM")]
    #[strum(to_string = "BANK_DIFFICULTY_MEDIUM")]
    Medium,
    #[serde(rename = "BANK_DIFFICULTY_HARD")]
    #[strum(to_string = "BANK_DIFFICULTY_HARD")]
    Hard,
    #[serde(rename = "BANK_DIFFICULTY_CRITICAL")]
    #[strum(to_string = "BANK_DIFFICULTY_CRITICAL")]
    Critical,
    #[serde(rename = "BANK_DIFFICULTY_BOSS")]
    #[strum(to_string = "BANK_DIFFICULTY_BOSS")]
    Boss    
}

impl From<BankDifficultyType> for bank_difficulty::BankDifficultyType {
    fn from(val: BankDifficultyType) -> Self {
        match val {
            BankDifficultyType::Boss => bank_difficulty::BankDifficultyType::Boss,
            BankDifficultyType::Critical => bank_difficulty::BankDifficultyType::Critical,
            BankDifficultyType::Easy => bank_difficulty::BankDifficultyType::Easy,
            BankDifficultyType::Hard => bank_difficulty::BankDifficultyType::Hard,
            BankDifficultyType::Medium => bank_difficulty::BankDifficultyType::Medium
        }
    }
}

impl From<bank_difficulty::BankDifficultyType> for BankDifficultyType {
    fn from(value: bank_difficulty::BankDifficultyType) -> Self {
        match value {
            bank_difficulty::BankDifficultyType::Boss => BankDifficultyType::Boss,
            bank_difficulty::BankDifficultyType::Critical => BankDifficultyType::Critical,
            bank_difficulty::BankDifficultyType::Easy => BankDifficultyType::Easy,
            bank_difficulty::BankDifficultyType::Hard => BankDifficultyType::Hard,
            bank_difficulty::BankDifficultyType::Medium => BankDifficultyType::Medium
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankDifficultyModel {
    pub id: i32,
    pub difficulty: BankDifficultyType,
    pub chance: i32
}

impl From<bank_difficulty::Model> for BankDifficultyModel {
    fn from(value: bank_difficulty::Model) -> Self {
        BankDifficultyModel {
            id: value.id,
            difficulty: BankDifficultyType::from(value.difficulty_type),
            chance: value.chance
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankVariantModel {
    pub id: i32,
    pub label: String,
    //pub chance: i32,
    pub difficulty: BankDifficultyType
}

impl From<bank_variant::Model> for BankVariantModel {
    fn from(value: bank_variant::Model) -> Self {
        BankVariantModel {
            id: value.id,
            //chance: value.chance,
            label: value.label,
            difficulty: BankDifficultyType::from(value.difficulty)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, EnumString, Display)]
pub enum CreatureSlotType {
    #[serde(rename = "CREATURE_SLOT_TYPE_TIER")]
    #[strum(to_string = "CREATURE_SLOT_TYPE_TIER")]
    Tier,
    #[serde(rename = "CREATURE_SLOT_TYPE_CONCRETE")]
    #[strum(to_string = "CREATURE_SLOT_TYPE_CONCRETE")]
    Concrete
}

impl From<CreatureSlotType> for models::bank_creature_entry::BankCreatureSlotType {
    fn from(val: CreatureSlotType) -> Self {
        match val {
            CreatureSlotType::Concrete => models::bank_creature_entry::BankCreatureSlotType::Concrete,
            CreatureSlotType::Tier => models::bank_creature_entry::BankCreatureSlotType::Tier
        }
    }
}