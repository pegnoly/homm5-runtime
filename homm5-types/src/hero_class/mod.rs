use serde::{Deserialize, Serialize};

use crate::common::FileRef;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillProb {
    pub SkillID: String,
    pub Prob: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillsProbs {
    #[serde(rename = "Item")]
    pub items: Option<Vec<SkillProb>>
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct HeroClassObject {
    pub NameFileRef: Option<FileRef>,
    pub SkillsProbs: Option<SkillsProbs>
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct HeroClass {
    pub ID: String,
    pub obj: HeroClassObject
}