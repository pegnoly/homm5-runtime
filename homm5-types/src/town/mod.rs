use serde::{Serialize, Deserialize};
use crate::{common::{FileRef, Pos, Trigger, ArmySlot}, Homm5Type, player::PlayerID};
use strum_macros::{EnumString, EnumIter, Display};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, EnumString, PartialEq, Eq, Hash, EnumIter, Display)]
pub enum TownType {
    #[serde(rename = "TOWN_NO_TYPE")]
    #[strum(to_string = "TOWN_NO_TYPE")]
    TownNoType,
    #[serde(rename = "TOWN_HEAVEN")]
    #[strum(to_string = "TOWN_HEAVEN")]
    TownHeaven,
    #[serde(rename = "TOWN_PRESERVE")]
    #[strum(to_string = "TOWN_PRESERVE")]
    TownPreserve,
    #[serde(rename = "TOWN_ACADEMY")]
    #[strum(to_string = "TOWN_ACADEMY")]
    TownAcademy,
    #[serde(rename = "TOWN_DUNGEON")]
    #[strum(to_string = "TOWN_DUNGEON")]
    TownDungeon,
    #[serde(rename = "TOWN_NECROMANCY")]
    #[strum(to_string = "TOWN_NECROMANCY")]
    TownNecromancy,
    #[serde(rename = "TOWN_INFERNO")]
    #[strum(to_string = "TOWN_INFERNO")]
    TownInferno,
    #[serde(rename = "TOWN_FORTRESS")]
    #[strum(to_string = "TOWN_FORTRESS")]
    TownFortress,
    #[serde(rename = "TOWN_STRONGHOLD")]
    #[strum(to_string = "TOWN_STRONGHOLD")]
    TownStronghold
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TownBuildingType {
    #[serde(rename = "TB_TOWN_HALL")]
    TownHall, 
    #[serde(rename = "TB_FORT")]
    Fort,
    #[serde(rename = "TB_MARKETPLACE")]
    Marketplace,
    #[serde(rename = "TB_SHIPYARD")]
    Shipyard,
    #[serde(rename = "TB_TAVERN")]
    Tavern,
    #[serde(rename = "TB_BLACKSMITH")]
    Blacksmith, 
    #[serde(rename = "TB_MAGIC_GUILD")]
    MagicGuild,
    #[serde(rename = "TB_DWELLING_1")]
    Dwelling1,
    #[serde(rename = "TB_DWELLING_2")]
    Dwelling2,
    #[serde(rename = "TB_DWELLING_3")]
    Dwelling3, 
    #[serde(rename = "TB_DWELLING_4")]
    Dwelling4,
    #[serde(rename = "TB_DWELLING_5")]
    Dwelling5,
    #[serde(rename = "TB_DWELLING_6")]
    Dwelling6,
    #[serde(rename = "TB_DWELLING_7")]
    Dwelling7,
    #[serde(rename = "TB_GRAIL")]
    Grail,
    #[serde(rename = "TB_WONDER")]
    Wonder,
    #[serde(rename = "TB_SPECIAL_0")]
    Special0,
    #[serde(rename = "TB_SPECIAL_1")]
    Special1,
    #[serde(rename = "TB_SPECIAL_2")]
    Special2,
    #[serde(rename = "TB_SPECIAL_3")]
    Special3,
    #[serde(rename = "TB_SPECIAL_4")]
    Special4,
    #[serde(rename = "TB_SPECIAL_5")]
    Special5,
    #[serde(rename = "TB_SPECIAL_6")]
    Special6,
    #[serde(rename = "TB_SPECIAL_7")]
    Special7,
    #[serde(rename = "TB_SPECIAL_8")]
    Special8,
    #[serde(rename = "TB_SPECIAL_9")]
    Special9,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TownBuildingLevel {
    #[serde(rename = "BLD_UPG_NONE")]
    BldUpgNone,
    #[serde(rename = "BLD_UPG_1")]
    BldUpg1,
    #[serde(rename = "BLD_UPG_2")]
    BldUpg2,
    #[serde(rename = "BLD_UPG_3")]
    BldUpg3,
    #[serde(rename = "BLD_UPG_4")]
    BldUpg4,
    #[serde(rename = "BLD_UPG_5")]
    BldUpg5
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipTile {
    pub x: u16,
    pub y: u16
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TownBuilding {
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub Type: TownBuildingType,
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub InitialUpgrade: TownBuildingLevel,
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub MaxUpgrade: TownBuildingLevel
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Editable {
    pub NameFileRef: crate::common::FileRef,
    pub BiographyFileRef: crate::common::FileRef
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreaturesUpgradesFilter{
    pub ForbiddenBasicUpgradeTiers: String,
    pub ForbiddenAlterUpgradeTiers: String,
    pub NotUpgradeable: String,
    pub ForbiddenUpgrades: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TownBuildings {
    #[serde(rename = "Item")]
    pub items: Vec<TownBuilding>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmySlots {
    #[serde(rename = "Item")]
    pub army_slots: Option<Vec<ArmySlot>>
}


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AdvMapTown {
    #[serde(rename = "Pos")]
    pub pos: Pos,
    #[serde(rename = "Rot")]
    pub rot: f32,
    #[serde(rename = "Floor")]
    pub floor: u8,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CombatScript")]
    pub combat_script: String,
    #[serde(rename = "pointLights")]
    pub point_lights: String,
    #[serde(rename = "Shared")]
    pub shared: FileRef,
    #[serde(rename = "PlayerID")]
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub player_id: PlayerID,
    #[serde(rename = "CaptureTrigger")]
    pub capture_trigger: Trigger,
    #[serde(rename = "HeroDeployTrigger")]
    pub hero_deploy_trigger: Trigger,
    #[serde(rename = "ShipTile")]
    pub ship_tile: ShipTile,
    #[serde(rename = "Specialization")]
    pub specialization: FileRef,
    #[serde(rename = "buildings")]
    pub buildings: TownBuildings,
    #[serde(rename = "Editable")]
    pub editable: Editable,
    #[serde(rename = "armySlots")]
    pub army_slots: ArmySlots,
    #[serde(rename = "spellIDs")]
    pub spell_ids: String,
    #[serde(rename = "CaptionFileRef")]
    pub caption_file_ref: FileRef,
    #[serde(rename = "GarrisonHero")]
    pub garrison_hero: String,
    #[serde(rename = "Script")]
    pub script: String,
    #[serde(rename = "RndSource")]
    pub rnd_source: String,
    #[serde(rename = "LinkToPlayer")]
    pub link_to_player: String,
    #[serde(rename = "LinkToTown")]
    pub link_to_town: String,
    #[serde(rename = "CanCaptureOnlyNotVisit")]
    pub can_capture_only_not_visit: bool,
    #[serde(rename = "AllowQuickCombat")]
    pub allow_quick_combat: bool,
    #[serde(rename = "CreaturesUpgradesFilter")]
    pub creatures_upgrades_filter: CreaturesUpgradesFilter,
    #[serde(rename = "GarrisonBlockedForAI")]
    pub garrison_blocked_for_ai: bool,
    #[serde(rename = "BannedRaces")]
    pub banned_races: String
}

impl Homm5Type for AdvMapTown {
}