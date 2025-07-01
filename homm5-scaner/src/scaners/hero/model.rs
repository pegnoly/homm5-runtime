use std::str::FromStr;

use crate::{
    core::ToLua,
    scaners::common::{HeroClass, Mastery, Town},
};
use homm5_types::hero::AdvMapHeroShared;
use sea_orm::{FromJsonQueryResult, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "heroes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub script_name: String,
    pub class: HeroClass,
    pub icon_xdb: String,
    pub specialization: String,
    pub primary_skill: SkillWithMasteryModel,
    pub spec_name_txt: String,
    pub spec_name: String,
    pub spec_desc_txt: String,
    pub spec_desc: String,
    pub spec_icon: String,
    pub icon: String,
    pub town: Town,
    pub editable: Editable,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct SkillWithMasteryModel {
    pub mastery: Mastery,
    pub skill: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct Editable {
    pub name_txt: String,
    pub name: String,
    pub bio_txt: String,
    pub bio: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<AdvMapHeroShared> for Model {
    fn from(value: AdvMapHeroShared) -> Self {
        Model {
            id: Default::default(),
            icon_xdb: if let Some(ref file) = value.FaceTexture {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            class: HeroClass::from_str(&value.Class).unwrap_or(HeroClass::None),
            specialization: value.Specialization,
            primary_skill: SkillWithMasteryModel {
                mastery: Mastery::from_str(&value.PrimarySkill.Mastery)
                    .unwrap_or(Mastery::MasteryNone),
                skill: value.PrimarySkill.SkillID,
            },
            script_name: value.InternalName,
            spec_name_txt: if let Some(ref file) = value.SpecializationNameFileRef {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            spec_name: Default::default(),
            spec_desc_txt: if let Some(ref file) = value.SpecializationDescFileRef {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            spec_desc: Default::default(),
            spec_icon: if let Some(ref file) = value.SpecializationIcon {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            icon: if let Some(ref file) = value.FaceTexture {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            town: Town::from_str(&value.TownType).unwrap_or(Town::TownNoType),
            editable: Editable {
                name_txt: if let Some(ref file) = value.Editable.NameFileRef {
                    file.href.clone().unwrap_or(String::new())
                } else {
                    String::new()
                },
                name: Default::default(),
                bio_txt: if let Some(ref file) = value.Editable.BiographyFileRef {
                    file.href.clone().unwrap_or(String::new())
                } else {
                    String::new()
                },
                bio: Default::default(),
            },
        }
    }
}

impl ToLua for Model {
    fn to_lua_string(&self) -> String {
        // let is_scenario_lua = if self.ScenarioHero == true {"1"} else {"nil"};
        format!(
            "\t[\"{}\"] = {{
        hero_class = {},
        spec = {},
        spec_name = \"{}\",
        spec_desc = \"{}\",
        spec_icon = \"{}\",
        icon = \"{}\",
        town = {},
        name = \"{}\",
        bio = \"{}\"
    }},\n",
            self.script_name,
            // is_scenario_lua,
            self.class,
            self.specialization,
            self.spec_name_txt,
            self.spec_desc_txt,
            self.spec_icon,
            self.icon,
            self.town,
            self.editable.name_txt,
            self.editable.bio_txt
        )
    }
}
