use std::str::FromStr;

use homm5_types::{art::{AdvMapArtifactShared, HeroStatsModif}, common::FileRef};
use sea_orm::{FromJsonQueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, FromRepr};

use crate::core::{ToJsonCompatibleString, ToLua};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "artifacts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name_txt: String,
    pub name: String,
    pub desc_txt: String,
    pub desc: String,
    pub slot: ArtifactSlotType,
    pub class: ArtifactClassType,
    pub icon_xdb: String,
    pub cost: i32,
    pub is_generatable: bool,
    pub game_id: String,
    pub attack: i32,
    pub defence: i32,
    pub spellpower: i32,
    pub knowledge: i32,
    pub luck: i32,
    pub morale: i32,
    pub unused_data: UnusedArtifactDataModel
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct UnusedArtifactDataModel {
    pub ai_value: i32,
    pub shared: Option<String>,
    pub model: Option<String>,
    pub available_for_presets: bool,
    pub preset_price: i32
}

#[derive(
    Debug,
    DeriveActiveEnum,
    EnumIter,
    EnumString,
    FromRepr,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    Hash,
    Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ArtifactSlotType {
    #[sea_orm(string_value = "PRIMARY")]
    #[serde(rename = "PRIMARY")]
    #[strum(serialize = "PRIMARY")]
    Primary,
    #[sea_orm(string_value = "SECONDARY")]
    #[serde(rename = "SECONDARY")]
    #[strum(serialize = "SECONDARY")]
    Secondary,
    #[sea_orm(string_value = "HEAD")]
    #[serde(rename = "HEAD")]
    #[strum(serialize = "HEAD")]
    Head,
    #[sea_orm(string_value = "MISCSLOT1")]
    #[serde(rename = "MISCSLOT1")]
    #[strum(serialize = "MISCSLOT1")]
    Miscslot1,
    #[sea_orm(string_value = "CHEST")]
    #[serde(rename = "CHEST")]
    #[strum(serialize = "CHEST")]
    Chest,
    #[sea_orm(string_value = "FINGER")]
    #[serde(rename = "FINGER")]
    #[strum(serialize = "FINGER")]
    Finger,
    #[sea_orm(string_value = "NECK")]
    #[serde(rename = "NECK")]
    #[strum(serialize = "NECK")]
    Neck,
    #[sea_orm(string_value = "FEET")]
    #[serde(rename = "FEET")]
    #[strum(serialize = "FEET")]
    Feet,
    #[sea_orm(string_value = "SHOULDERS")]
    #[serde(rename = "SHOULDERS")]
    #[strum(serialize = "SHOULDERS")]
    Shoulders,
    #[sea_orm(string_value = "INVENTORY")]
    #[serde(rename = "INVENTORY")]
    #[strum(serialize = "INVENTORY")]
    Inventory,
}

impl ToJsonCompatibleString for ArtifactSlotType {
    fn to_json_compatible_repr(&self) -> &str {
        match self {
            ArtifactSlotType::Primary => "Primary",
            ArtifactSlotType::Secondary => "Secondary",
            ArtifactSlotType::Head => "Head",
            ArtifactSlotType::Miscslot1 => "MiscSlot",
            ArtifactSlotType::Chest => "Chest",
            ArtifactSlotType::Finger => "Finger",
            ArtifactSlotType::Neck => "Neck",
            ArtifactSlotType::Feet => "Feet",
            ArtifactSlotType::Shoulders => "Shoulders",
            ArtifactSlotType::Inventory => "Inventory",
        }
    }
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
pub enum ArtifactClassType {
    #[sea_orm(string_value = "ARTF_CLASS_MINOR")]
    #[serde(rename = "ARTF_CLASS_MINOR")]
    #[strum(serialize = "ARTF_CLASS_MINOR")]
    Minor,
    #[sea_orm(string_value = "ARTF_CLASS_MAJOR")]
    #[serde(rename = "ARTF_CLASS_MAJOR")]
    #[strum(serialize = "ARTF_CLASS_MAJOR")]
    Major,
    #[sea_orm(string_value = "ARTF_CLASS_RELIC")]
    #[serde(rename = "ARTF_CLASS_RELIC")]
    #[strum(serialize = "ARTF_CLASS_RELIC")]
    Relic,
    #[sea_orm(string_value = "ARTF_CLASS_GRAIL")]
    #[serde(rename = "ARTF_CLASS_GRAIL")]
    #[strum(serialize = "ARTF_CLASS_GRAIL")]
    Grail,
}

impl ToJsonCompatibleString for ArtifactClassType {
    fn to_json_compatible_repr(&self) -> &str {
        match self {
            ArtifactClassType::Minor => "Minor",
            ArtifactClassType::Major => "Major",
            ArtifactClassType::Relic => "Relic",
            ArtifactClassType::Grail => "Grail",
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<AdvMapArtifactShared> for Model {
    fn from(value: AdvMapArtifactShared) -> Self {
        Model {
            id: Default::default(),
            name_txt: if let Some(ref file) = value.NameFileRef {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            name: String::new(),
            desc_txt: if let Some(ref file) = value.DescriptionFileRef {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            desc: String::new(),
            slot: ArtifactSlotType::from_str(&value.Slot).unwrap_or(ArtifactSlotType::Inventory),
            class: ArtifactClassType::from_str(&value.Type).unwrap_or(ArtifactClassType::Grail),
            icon_xdb: if let Some(ref file) = value.Icon {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            cost: value.CostOfGold as i32,
            is_generatable: value.CanBeGeneratedToSell,
            game_id: Default::default(),
            attack: value.HeroStatsModif.Attack,
            defence: value.HeroStatsModif.Defence,
            spellpower: value.HeroStatsModif.SpellPower,
            knowledge: value.HeroStatsModif.Knowledge,
            luck: value.HeroStatsModif.Luck,
            morale: value.HeroStatsModif.Morale,
            unused_data: UnusedArtifactDataModel { 
                ai_value: value.AIValue, 
                shared: if let Some(shared) = value.ArtifactShared {
                    shared.href
                } else {
                    None
                }, 
                model: if let Some(model) = value.Model {
                    model.href
                } else {
                    None
                }, 
                available_for_presets: value.AvailableForPresets, 
                preset_price: value.PresetPrice 
            }
        }
    }
}

impl From<Model> for AdvMapArtifactShared {
    fn from(value: Model) -> Self {
        AdvMapArtifactShared {
            AIValue: value.unused_data.ai_value,
            NameFileRef: Some(FileRef { href: Some(format!("/{}", value.name_txt.replace("\\", "/"))) }),
            DescriptionFileRef: Some(FileRef { href: Some(format!("/{}",value.desc_txt.replace("\\", "/"))) }),
            Model: value.unused_data.model.map(|model| FileRef { href: Some(model) }),
            Type: value.class.to_string(),
            Slot: value.slot.to_string(),
            Icon: Some(FileRef { href: Some( if value.icon_xdb.is_empty() {
                value.icon_xdb
            } else {
                format!("/{}#xpointer(/Texture)", value.icon_xdb.replace("\\", "/"))
            }) }),
            CostOfGold: value.cost as u32,
            CanBeGeneratedToSell: value.is_generatable,
            HeroStatsModif: HeroStatsModif {
                Attack: value.attack,
                Defence: value.defence,
                Knowledge: value.knowledge,
                SpellPower: value.spellpower,
                Morale: value.morale,
                Luck: value.luck
            },
            ArtifactShared: value.unused_data.shared.map(|shared| FileRef { href: Some(shared) }),
            AvailableForPresets: value.unused_data.available_for_presets,
            PresetPrice: value.unused_data.preset_price
        }
    }
}

impl ToLua for Model {
    fn to_lua_string(&self) -> String {
        let is_sellable = if self.is_generatable { "1" } else { "nil" };
        format!(
            "\t[{}] = {{
        is_sellable = {},
        name = \"{}\",
        desc = \"{}\",
        icon = \"{}\",
        cost = {},
        slot = {},
        type = {}
    }},\n",
            self.id,
            is_sellable,
            self.name_txt,
            self.desc_txt,
            self.icon_xdb,
            self.cost,
            self.slot,
            self.class
        )
    }
}
