use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, Display};
use crate::{common::{FileRef, Pos, Trigger}, Homm5Type, town::CreaturesUpgradesFilter};

#[derive(Debug, Serialize, Deserialize, PartialEq, Hash, Eq, EnumString, Clone, Copy, Display)]
#[allow(non_camel_case_types)]
pub enum BankType {
    BANK_NAGA_TEMPLE,
    BANK_DEMOLISH,
    BANK_SUNKEN_TEMPLE,
    BANK_CRYPT,
    BANK_UNKEMPT,
    BANK_DRAGON_UTOPIA,
    BANK_ELEMENTAL_STOCKPILE,
    BANK_DWARVEN_TREASURE,
    BANK_MAGI_VAULT,
    BANK_PYRAMID,
    BANK_TREANT_THICKET,
    BANK_BLOOD_TEMPLE,
    BANK_GARGOYLE_STONEVAULT,
    BANK_NAMTARU_TEMPLE,
    BANK_SNOW_APE_CAVE,
    BANK_BTDSUNKENTEMPLE
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Hash, Eq, EnumString, Clone, Copy, Display)]
#[allow(non_camel_case_types)]
pub enum NewBuildingType {
    BTD_STATUE_OF_REVELATION,
    BTD_SUN_RIDER_MONUMENT,
    BTD_WARMEN_HOUSE,
    BTD_KNOWLEDGE_MEGALITH,
    BTD_DWARVEN_MINE
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvMapBuilding {
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
    #[serde(rename = "PlayerID")]
    pub player_id: String,
    #[serde(rename = "CaptureTrigger")]
    pub capture_trigger: Trigger,
    #[serde(rename = "GroupID")]
    pub group_id: u32,
    #[serde(rename = "showCameras")]
    pub show_cameras: String
}

impl Homm5Type for AdvMapBuilding {
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AdvMapShrine {
    pub Pos: Option<Pos>,
    pub Rot: f32,
    pub Floor: u16,
    pub Name: String,
    pub CombatScript: String,
    pub pointLights: String,
    pub Shared: FileRef,
    pub SpellID: String
}

impl Homm5Type for AdvMapShrine {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvMapHillFort {
    #[serde(rename = "Pos")]
    pub pos: Option<Pos>,
    #[serde(rename = "Rot")]
    pub rot: f32,
    #[serde(rename = "Floor")]
    pub floor: u16,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CombatScript")]
    pub combat_script: String,
    #[serde(rename = "pointLights")]
    pub point_lights: String,
    #[serde(rename = "Shared")]
    pub shared: FileRef,
    #[serde(rename = "CreaturesUpgradesFilter")]
    pub creatures_upgrades_filter: CreaturesUpgradesFilter
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdvMapStatic {
    #[serde(rename = "Pos")]
    pub pos: Option<Pos>,
    #[serde(rename = "Rot")]
    pub rot: f32,
    #[serde(rename = "Floor")]
    pub floor: u16,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CombatScript")]
    pub combat_script: String,
    #[serde(rename = "pointLights")]
    pub point_lights: String,
    #[serde(rename = "Shared")]
    pub shared: FileRef,
    #[serde(rename = "IsRemovable")]
    pub is_removable: bool,
    #[serde(rename = "TerrainAligned")]
    pub terrain_aligned: bool,
    #[serde(rename = "ScalePercent")]
    pub scale_percent: u16
}