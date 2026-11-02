use crate::{
    common::{Color, Trigger},
    Homm5Type,
};
use serde::{Deserialize, Serialize};

/// Rectangle area (in map cells) covered by a region
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct RegionRect {
    #[serde(rename = "x1")]
    pub x1: u16,
    #[serde(rename = "y1")]
    pub y1: u16,
    #[serde(rename = "x2")]
    pub x2: u16,
    #[serde(rename = "y2")]
    pub y2: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Region {
    #[serde(rename = "Floor")]
    pub floor: u16,
    #[serde(rename = "Rect")]
    pub rect: RegionRect,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Color")]
    pub color: Color,
    #[serde(rename = "EnterAndStopTrigger")]
    pub enter_and_stop_trigger: Trigger,
    #[serde(rename = "EnterWithoutStopTrigger")]
    pub enter_without_stop_trigger: Trigger,
    #[serde(rename = "ExitTrigger")]
    pub exit_trigger: Trigger,
}

impl Homm5Type for Region {}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Regions {
    #[serde(rename = "Item")]
    pub items: Option<Vec<Region>>,
}
