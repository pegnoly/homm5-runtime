use homm5_scaner::prelude::Mastery;
use sea_orm::{prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "reserve_heroes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub xdb_path: String,
    pub primary_skill: BaseSkill,
    pub skills: ReserveHeroSkills,
    pub perks: ReserveHeroPerks,
    pub spells: ReserveHeroSpells
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct BaseSkill {
    pub skill: String,
    pub mastery: Mastery
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct ReserveHeroSkills {
    pub skills: Vec<BaseSkill>
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct ReserveHeroPerks {
    pub perks: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct ReserveHeroSpells {
    pub spells: Vec<String>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}