use std::str::FromStr;

use homm5_types::spell::SpellShared;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::core::{ToJsonCompatibleString, ToLua};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "spells")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name_txt: String,
    pub name: String,
    pub desc_txt: String,
    pub desc: String,
    pub icon_xdb: String,
    pub cost: i32,
    pub level: i32,
    pub school: MagicSchoolType,
    pub is_aimed: bool,
    pub is_area: bool,
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
pub enum MagicSchoolType {
    #[sea_orm(string_value = "MAGIC_SCHOOL_NONE")]
    #[serde(rename = "MAGIC_SCHOOL_NONE")]
    #[strum(serialize = "MAGIC_SCHOOL_NONE")]
    None,
    #[sea_orm(string_value = "MAGIC_SCHOOL_SPECIAL")]
    #[serde(rename = "MAGIC_SCHOOL_SPECIAL")]
    #[strum(serialize = "MAGIC_SCHOOL_SPECIAL")]
    Special,
    #[sea_orm(string_value = "MAGIC_SCHOOL_LIGHT")]
    #[serde(rename = "MAGIC_SCHOOL_LIGHT")]
    #[strum(serialize = "MAGIC_SCHOOL_LIGHT")]
    Light,
    #[sea_orm(string_value = "MAGIC_SCHOOL_DARK")]
    #[serde(rename = "MAGIC_SCHOOL_DARK")]
    #[strum(serialize = "MAGIC_SCHOOL_DARK")]
    Dark,
    #[sea_orm(string_value = "MAGIC_SCHOOL_DESTRUCTIVE")]
    #[serde(rename = "MAGIC_SCHOOL_DESTRUCTIVE")]
    #[strum(serialize = "MAGIC_SCHOOL_DESTRUCTIVE")]
    Destructive,
    #[sea_orm(string_value = "MAGIC_SCHOOL_SUMMONING")]
    #[serde(rename = "MAGIC_SCHOOL_SUMMONING")]
    #[strum(serialize = "MAGIC_SCHOOL_SUMMONING")]
    Summoning,
    #[sea_orm(string_value = "MAGIC_SCHOOL_ADVENTURE")]
    #[serde(rename = "MAGIC_SCHOOL_ADVENTURE")]
    #[strum(serialize = "MAGIC_SCHOOL_ADVENTURE")]
    Adventure,
    #[sea_orm(string_value = "MAGIC_SCHOOL_RUNIC")]
    #[serde(rename = "MAGIC_SCHOOL_RUNIC")]
    #[strum(serialize = "MAGIC_SCHOOL_RUNIC")]
    Runic,
    #[sea_orm(string_value = "MAGIC_SCHOOL_WARCRIES")]
    #[serde(rename = "MAGIC_SCHOOL_WARCRIES")]
    #[strum(serialize = "MAGIC_SCHOOL_WARCRIES")]
    Warcries,
}

impl ToJsonCompatibleString for MagicSchoolType {
    fn to_json_compatible_repr(&self) -> &str {
        match self {
            MagicSchoolType::Adventure => "Adventure",
            MagicSchoolType::Dark => "Dark",
            MagicSchoolType::None => "None",
            MagicSchoolType::Special => "Special",
            MagicSchoolType::Light => "Light",
            MagicSchoolType::Destructive => "Destructive",
            MagicSchoolType::Summoning => "Summoning",
            MagicSchoolType::Runic => "Runic",
            MagicSchoolType::Warcries => "Warcries",
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<SpellShared> for Model {
    fn from(value: SpellShared) -> Self {
        Model {
            id: Default::default(),
            name_txt: if let Some(ref file) = value.NameFileRef {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            name: Default::default(),
            desc_txt: if let Some(ref file) = value.LongDescriptionFileRef {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            desc: Default::default(),
            icon_xdb: if let Some(ref file) = value.Texture {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            cost: value.TrainedCost as i32,
            level: value.Level as i32,
            school: MagicSchoolType::from_str(&value.MagicSchool).unwrap_or(MagicSchoolType::None),
            is_aimed: value.IsAimed,
            is_area: value.IsAreaAttack,
        }
    }
}

impl ToLua for Model {
    fn to_lua_string(&self) -> String {
        let is_aimed = if self.is_aimed == true { "1" } else { "nil" };
        let is_area = if self.is_area == true { "1" } else { "nil" };
        format!(
            "\t[{}] = {{
        name = \"{}\",
        desc = \"{}\",
        icon = \"{}\",
        school = {},
        level = {},
        is_aimed = {},
        is_area = {}
    }},\n",
            self.id,
            self.name_txt,
            self.desc_txt,
            self.icon_xdb,
            self.school,
            self.level,
            is_aimed,
            is_area
        )
    }
}
