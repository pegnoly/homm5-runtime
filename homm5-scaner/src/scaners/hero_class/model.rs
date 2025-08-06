use homm5_types::{common::FileRef, hero_class::HeroClass};
use sea_orm::{prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "hero_classes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub game_id: String,
    pub name_txt: String,
    pub name: String,
    pub skills: ClassSkills
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct ClassSkills {
    pub skills: Vec<String>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<HeroClass> for Model {
    fn from(value: HeroClass) -> Self {
        Model { 
            id: Default::default(), 
            game_id: value.ID, 
            name_txt: value.obj.NameFileRef.unwrap_or(FileRef { href: None }).href.unwrap_or_default(), 
            name: Default::default(), 
            skills: if let Some(skills_data) = value.obj.SkillsProbs {
                if let Some(probs_data) = skills_data.items {
                    ClassSkills { skills: Vec::from_iter(probs_data.iter().map(|prob| prob.SkillID.clone())) }
                } else {
                    ClassSkills { skills: vec![] }
                }
            } else {
                ClassSkills { skills: vec![] }
            }
        }
    }
}