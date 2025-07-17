use serde::{Deserialize, Serialize};

use crate::common::FileRef;

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillNames {
    #[serde(rename = "Item")]
    pub names: Option<Vec<FileRef>>
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillObject {
    pub NameFileRef: SkillNames,
    pub HeroClass: String,
    pub BasicSkillID: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub ID: String,
    pub obj: SkillObject
}