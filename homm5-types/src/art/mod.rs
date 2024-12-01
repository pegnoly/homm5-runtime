use serde::{Serialize, Deserialize};
use crate::common::FileRef;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AdvMapArtifactShared {
    pub NameFileRef: Option<FileRef>,
    pub DescriptionFileRef: Option<FileRef>,
    pub Type: String,
    pub Slot: String,
    pub Icon: Option<FileRef>,
    pub CostOfGold: u32,
    pub CanBeGeneratedToSell: bool
}