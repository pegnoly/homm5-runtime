use sea_orm::{FromJsonQueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
    Debug,
    DeriveActiveEnum,
    EnumIter,
    EnumString,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum Town {
    #[sea_orm(string_value = "TOWN_NO_TYPE")]
    #[serde(rename = "TOWN_NO_TYPE")]
    #[strum(serialize = "TOWN_NO_TYPE")]
    TownNoType,
    #[sea_orm(string_value = "TOWN_HEAVEN")]
    #[serde(rename = "TOWN_HEAVEN")]
    #[strum(serialize = "TOWN_HEAVEN")]
    TownHeaven,
    #[sea_orm(string_value = "TOWN_PRESERVE")]
    #[serde(rename = "TOWN_PRESERVE")]
    #[strum(serialize = "TOWN_PRESERVE")]
    TownPreserve,
    #[sea_orm(string_value = "TOWN_ACADEMY")]
    #[serde(rename = "TOWN_ACADEMY")]
    #[strum(serialize = "TOWN_ACADEMY")]
    TownAcademy,
    #[sea_orm(string_value = "TOWN_DUNGEON")]
    #[serde(rename = "TOWN_DUNGEON")]
    #[strum(serialize = "TOWN_DUNGEON")]
    TownDungeon,
    #[sea_orm(string_value = "TOWN_NECROMANCY")]
    #[serde(rename = "TOWN_NECROMANCY")]
    #[strum(serialize = "TOWN_NECROMANCY")]
    TownNecromancy,
    #[sea_orm(string_value = "TOWN_INFERNO")]
    #[serde(rename = "TOWN_INFERNO")]
    #[strum(serialize = "TOWN_INFERNO")]
    TownInferno,
    #[sea_orm(string_value = "TOWN_FORTRESS")]
    #[serde(rename = "TOWN_FORTRESS")]
    #[strum(serialize = "TOWN_FORTRESS")]
    TownFortress,
    #[sea_orm(string_value = "TOWN_STRONGHOLD")]
    #[serde(rename = "TOWN_STRONGHOLD")]
    #[strum(serialize = "TOWN_STRONGHOLD")]
    TownStronghold,
}

#[derive(
    Debug,
    DeriveActiveEnum,
    EnumIter,
    EnumString,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum HeroClass {
    #[sea_orm(string_value = "HERO_CLASS_NONE")]
    #[serde(rename = "HERO_CLASS_NONE")]
    #[strum(serialize = "HERO_CLASS_NONE")]
    None,
    #[sea_orm(string_value = "HERO_CLASS_KNIGHT")]
    #[serde(rename = "HERO_CLASS_KNIGHT")]
    #[strum(serialize = "HERO_CLASS_KNIGHT")]
    Knight,
    #[sea_orm(string_value = "HERO_CLASS_DEMON_LORD")]
    #[serde(rename = "HERO_CLASS_DEMON_LORD")]
    #[strum(serialize = "HERO_CLASS_DEMON_LORD")]
    DemonLord,
    #[sea_orm(string_value = "HERO_CLASS_NECROMANCER")]
    #[serde(rename = "HERO_CLASS_NECROMANCER")]
    #[strum(serialize = "HERO_CLASS_NECROMANCER")]
    Necromancer,
    #[sea_orm(string_value = "HERO_CLASS_RANGER")]
    #[serde(rename = "HERO_CLASS_RANGER")]
    #[strum(serialize = "HERO_CLASS_RANGER")]
    Ranger,
    #[sea_orm(string_value = "HERO_CLASS_WARLOCK")]
    #[serde(rename = "HERO_CLASS_WARLOCK")]
    #[strum(serialize = "HERO_CLASS_WARLOCK")]
    Warlock,
    #[sea_orm(string_value = "HERO_CLASS_WIZARD")]
    #[serde(rename = "HERO_CLASS_WIZARD")]
    #[strum(serialize = "HERO_CLASS_WIZARD")]
    Wizard,
    #[sea_orm(string_value = "HERO_CLASS_RUNEMAGE")]
    #[serde(rename = "HERO_CLASS_RUNEMAGE")]
    #[strum(serialize = "HERO_CLASS_RUNEMAGE")]
    Runemage,
    #[sea_orm(string_value = "HERO_CLASS_BARBARIAN")]
    #[serde(rename = "HERO_CLASS_BARBARIAN")]
    #[strum(serialize = "HERO_CLASS_BARBARIAN")]
    Barbarian,
}

#[derive(
    Debug,
    DeriveActiveEnum,
    EnumIter,
    EnumString,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum Mastery {
    #[sea_orm(string_value = "MASTERY_NONE")]
    #[serde(rename = "MASTERY_NONE")]
    #[strum(serialize = "MASTERY_NONE")]
    MasteryNone,
    #[sea_orm(string_value = "MASTERY_BASIC")]
    #[serde(rename = "MASTERY_BASIC")]
    #[strum(serialize = "MASTERY_BASIC")]
    MasteryBasic,
    #[sea_orm(string_value = "MASTERY_ADVANCED")]
    #[serde(rename = "MASTERY_ADVANCED")]
    #[strum(serialize = "MASTERY_ADVANCED")]
    MasteryAdvanced,
    #[sea_orm(string_value = "MASTERY_EXPERT")]
    #[serde(rename = "MASTERY_EXPERT")]
    #[strum(serialize = "MASTERY_EXPERT")]
    MasteryExpert,
    #[sea_orm(string_value = "MASTERY_EXTRA_EXPERT")]
    #[serde(rename = "MASTERY_EXTRA_EXPERT")]
    #[strum(serialize = "MASTERY_EXTRA_EXPERT")]
    MasteryExtraExpert,
}

#[derive(
    Debug,
    DeriveActiveEnum,
    EnumIter,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    EnumString,
    Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MagicElement {
    #[sea_orm(string_value = "ELEMENT_NONE")]
    #[serde(rename = "ELEMENT_NONE")]
    #[strum(serialize = "ELEMENT_NONE")]
    ElementNone,
    #[sea_orm(string_value = "ELEMENT_FIRE")]
    #[serde(rename = "ELEMENT_FIRE")]
    #[strum(serialize = "ELEMENT_FIRE")]
    ElementFire,
    #[sea_orm(string_value = "ELEMENT_AIR")]
    #[serde(rename = "ELEMENT_AIR")]
    #[strum(serialize = "ELEMENT_AIR")]
    ElementAir,
    #[sea_orm(string_value = "ELEMENT_WATER")]
    #[serde(rename = "ELEMENT_WATER")]
    #[strum(serialize = "ELEMENT_WATER")]
    ElementWater,
    #[sea_orm(string_value = "ELEMENT_EARTH")]
    #[serde(rename = "ELEMENT_EARTH")]
    #[strum(serialize = "ELEMENT_EARTH")]
    ElementEarth,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ResourcesModel {
    pub wood: i32,
    pub ore: i32,
    pub mercury: i32,
    pub sulfur: i32,
    pub crystal: i32,
    pub gem: i32,
    pub gold: i32,
}
