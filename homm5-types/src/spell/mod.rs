use crate::common::FileRef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SpellShared {
    pub NameFileRef: Option<FileRef>,
    pub LongDescriptionFileRef: Option<FileRef>,
    pub Texture: Option<FileRef>,
    pub TrainedCost: u8,
    pub Level: u8,
    pub MagicSchool: String,
    pub IsAimed: bool,
    pub IsAreaAttack: bool,
}
