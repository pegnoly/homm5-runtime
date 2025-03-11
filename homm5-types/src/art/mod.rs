use crate::{common::{FileRef, PointLights, Pos}, town::ArmySlots};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AdvMapArtifactShared {
    pub NameFileRef: Option<FileRef>,
    pub DescriptionFileRef: Option<FileRef>,
    pub Type: String,
    pub Slot: String,
    pub Icon: Option<FileRef>,
    pub CostOfGold: u32,
    pub CanBeGeneratedToSell: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvMapArtifact {
    #[serde(rename = "Pos")]
    pub pos: Pos,
    #[serde(rename = "Rot")]
    pub rot: f32,
    #[serde(rename = "Floor")]
    pub floor: u8,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "CombatScript")]
    pub combat_script: Option<String>,
    #[serde(rename = "pointLights")]
    pub point_lights: Option<PointLights>,
    #[serde(rename = "Shared")]
    pub shared: FileRef,
    #[serde(rename = "armySlots")]
    pub army_slots: Option<ArmySlots>,
    #[serde(rename = "MessageFileRef")]
    pub message_file_ref: Option<FileRef>,
    #[serde(rename = "spellID")]
    pub spell_id: String,
    #[serde(rename = "RandomShiftRadius")]
    pub random_shift_radius: u32,
    #[serde(rename = "untransferable")]
    pub untransferable: bool
}
