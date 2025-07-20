use homm5_scaner::prelude::Mastery;
use sea_orm::{prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "reserve_heroes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub map_id: i32,
    pub player_id: i32,
    pub name: String,
    pub xdb_path: String,
    pub skills: ReserveHeroSkills,
    pub spells: ReserveHeroSpells
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct BaseSkill {
    pub slot: i32,
    pub skill: String,
    pub mastery: Mastery,
    pub perks: Vec<String>
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