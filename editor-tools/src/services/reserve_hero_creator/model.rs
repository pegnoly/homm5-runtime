use homm5_scaner::prelude::Mastery;
use homm5_types::{common::{ArmySlot, FileRef}, hero::{AdvMapHero, Editable, Perks, Skill, Skills, SpellIds}, player::PlayerID, town::ArmySlots};
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

impl From<Model> for AdvMapHero {
    fn from(value: Model) -> Self {
        AdvMapHero {
            player_id: PlayerID::PlayerNone,
            army_slots: Some(ArmySlots { army_slots: Some( vec![ {ArmySlot { creature: String::from("CREATURE_1000"), count: 1}}])}),
            editable: Editable {
                skills: Some(Skills { 
                    items: Some(
                        Vec::from_iter(value.skills.skills.iter()
                            .map(|s| Skill { Mastery: s.mastery.to_string(), SkillID: s.skill.clone()} )
                        ))
                    }),
                perkIDs: Some(Perks {
                    items: Some(
                        Vec::from_iter(value.skills.skills.iter()
                            .flat_map(|s| s.perks.clone())
                    ))
                }),
                spellIDs: Some(SpellIds {
                    items: Some(
                        Vec::from_iter(value.spells.spells.iter().cloned()
                    ))
                }),
                ..Default::default()
            },
            primary_skill_mastery: value.skills.skills.iter().find(|s| s.slot == 0).unwrap().mastery.to_string(),
            shared: FileRef { href: Some(format!("/{}#xpointer(/AdvMapHeroShared)", &value.xdb_path))},
            ..Default::default()
        }
    }
}