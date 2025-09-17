use std::sync::LazyLock;

use sea_orm::{FromJsonQueryResult, prelude::*};
use serde::{Deserialize, Serialize};

pub static BASE_SKILLS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        "HERO_SKILL_LOGISTICS",
        "HERO_SKILL_WAR_MACHINES",
        "HERO_SKILL_LEARNING",
        "HERO_SKILL_LEADERSHIP",
        "HERO_SKILL_LUCK",
        "HERO_SKILL_OFFENCE",
        "HERO_SKILL_DEFENCE",
        "HERO_SKILL_SORCERY",
        "HERO_SKILL_DESTRUCTIVE_MAGIC",
        "HERO_SKILL_DARK_MAGIC",
        "HERO_SKILL_LIGHT_MAGIC",
        "HERO_SKILL_SUMMONING_MAGIC",
        "HERO_SKILL_TRAINING",
        "HERO_SKILL_GATING",
        "HERO_SKILL_NECROMANCY",
        "HERO_SKILL_AVENGER",
        "HERO_SKILL_ARTIFICIER",
        "HERO_SKILL_INVOCATION",
        "HERO_SKILL_RUNELORE",
        "HERO_SKILL_DEMONIC_RAGE",
        "HERO_SKILL_VOICE",
        "HERO_SKILL_SHATTER_DESTRUCTIVE_MAGIC",
        "HERO_SKILL_SHATTER_DARK_MAGIC",
        "HERO_SKILL_SHATTER_LIGHT_MAGIC",
        "HERO_SKILL_SHATTER_SUMMONING_MAGIC",
        "HERO_SKILL_BARBARIAN_LEARNING",
    ]
});

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "skills")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub game_id: String,
    pub name_paths: NamePaths,
    pub names: Names,
    pub hero_class: String,
    pub basic_skill: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct Names {
    pub names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct NamePaths {
    pub paths: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
