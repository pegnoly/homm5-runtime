use serde::{Serialize, Deserialize};
use crate::{common::{FileRef, Pos}, Homm5Type};

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvMapTreasure {
    #[serde(rename = "Pos")]
    pub pos: Option<Pos>,
    #[serde(rename = "Rot")]
    pub rot: f32,
    #[serde(rename = "Floor")]
    pub floor: f32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CombatScript")]
    pub combat_script: String,
    #[serde(rename = "pointLights")]
    pub point_lights: String,
    #[serde(rename = "Shared")]
    pub shared: FileRef,
    #[serde(rename = "IsCustom")]
    pub is_custom: bool,
    #[serde(rename = "Amount")]
    pub amount: u32,
    #[serde(rename = "MessageFileRef")]
    pub message_file_ref: FileRef
}

impl Homm5Type for AdvMapTreasure {
}