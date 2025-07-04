use serde::{Deserialize, Serialize};

use crate::common::FileRef;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityObject {
    pub NameFileRef: Option<FileRef>,
    pub DescriptionFileRef: Option<FileRef>
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityShared {
    pub ID: String,
    pub obj: AbilityObject
}