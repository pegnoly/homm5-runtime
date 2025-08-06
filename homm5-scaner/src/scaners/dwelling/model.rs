use homm5_types::common::FileRef;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize, EnumString, Display, PartialEq, Eq, Hash, Clone)]
pub enum DwellingType {
    HumanT1Dwelling,
    HumanT2Dwelling,
    HumanT3Dwelling,
    HumansDwelling,
    InfernoT1Dwelling,
    InfernoT2Dwelling,
    InfernoT3Dwelling,
    InfernoDwelling,
    NecropolisT1Dwelling,
    NecropolisT2Dwelling,
    NecropolisT3Dwelling,
    NecropolisDwelling,
    ElvesT1Dwelling,
    ElvesT2Dwelling,
    ElvesT3Dwelling,
    ElvesDwelling,
    LigaT1Dwelling,
    LigaT2Dwelling,
    LigaT3Dwelling,
    LigaDwelling,
    MagesT1Dwelling,
    MagesT2Dwelling,
    MagesT3Dwelling,
    MagesDwelling,
    DwarfsT1Dwelling,
    DwarfsT2Dwelling,
    DwarfsT3Dwelling,
    DwarfsDwelling,
    HordeT1Dwelling,
    HordeT2Dwelling,
    HordeT3Dwelling,
    HordeDwelling,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockedTiles {
    #[serde(rename = "Item")]
    pub items: Option<Vec<Tile>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveTiles {
    #[serde(rename = "Item")]
    pub items: Option<Vec<Tile>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagesFileRef {
    #[serde(rename = "Item")]
    pub items: Option<Vec<FileRef>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dwelling {
    #[serde(rename = "Model")]
    pub model: Option<FileRef>,
    #[serde(rename = "blockedTiles")]
    pub blocked_tiles: Option<BlockedTiles>,
    #[serde(rename = "activeTiles")]
    pub active_tiles: Option<ActiveTiles>,
    #[serde(rename = "PossessionMarkerTile")]
    pub possession_tile_marker: Tile,
    #[serde(rename = "Effect")]
    pub effect: Option<FileRef>,
    #[serde(rename = "messagesFileRef")]
    pub messages_file_ref: Option<MessagesFileRef>,
    #[serde(rename = "Icon128")]
    pub icon: Option<FileRef>,
}