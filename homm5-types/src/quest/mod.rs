use serde::{Serialize, Deserialize};
use crate::common::{ArmySlot, SkillMastery, FileRef, Trigger};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "cell")]
pub struct Cell {
    pub x: u8,
    pub y: u8
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Coordinates {
    #[serde(rename = "FloorID")]
    pub floor_id: u8,
    pub cell: Cell
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Target {
    #[serde(rename = "Type")]
    pub _type: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Coords")]
    pub coords: Coordinates
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TargetGlance {
    #[serde(rename = "Target")]
    pub target: Target,
    #[serde(rename = "Radius")]
    pub radius: u16,
    #[serde(rename = "Duration")]
    pub duration: u32
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Resource {
    #[serde(rename = "Wood")]
    pub wood: u16,
    #[serde(rename = "Ore")]
    pub ore: u16,
    #[serde(rename = "Mercury")]
    pub mercury: u16,
    #[serde(rename = "Crystal")]
    pub crystal: u16,
    #[serde(rename = "Sulfur")]
    pub sulfur: u16,
    #[serde(rename = "Gem")]
    pub gem: u16,
    #[serde(rename = "Gold")]
    pub gold: u16
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Award {
    #[serde(rename = "Type")]
    pub _type: String,
    #[serde(rename = "Experience")]
    pub experience: u32,
    #[serde(rename = "Resources")]
    pub resources: Resource,
    #[serde(rename = "Attribute")]
    pub attribute: String,
    #[serde(rename = "AttributeAmount")]
    pub attribute_amount: u16,
    #[serde(rename = "ArtifactID")]
    pub artifact_id: String,
    #[serde(rename = "SpellID")]
    pub spell_id: String,
    #[serde(rename = "ArmySlot")]
    pub army_slot: ArmySlot,
    #[serde(rename = "SpellPoints")]
    pub spell_points: u16,
    #[serde(rename = "Morale")]
    pub morale: u8,
    #[serde(rename = "Luck")]
    pub luck: u8,
    #[serde(rename = "SkillWithMastery")]
    pub skill_with_mastery: SkillMastery,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "Item")]
#[allow(non_snake_case)]
pub struct Quest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CaptionFileRef")]
    pub caption_file_ref: FileRef,
    #[serde(rename = "ObscureCaptionFileRef")]
    pub obscure_caption_file_ref: FileRef,
    #[serde(rename = "DescriptionFileRef")]
    pub description_file_ref: FileRef,
    #[serde(rename = "ProgressCommentsFileRef")]
    pub progress_comments_file_ref: FileRef,
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "Parameters")]
    pub parameters: String,
    #[serde(rename = "Timeout")]
    pub timeout: i8,
    #[serde(rename = "Holdout")]
    pub holdout: i8,
    #[serde(rename = "CheckDelay")]
    pub check_delay: i8,
    #[serde(rename = "Dependencies")]
    pub dependencies: String,
    #[serde(rename = "InstantVictory")]
    pub instant_victory: bool,
    #[serde(rename = "TargetGlance")]
    pub target_glance: TargetGlance,
    #[serde(rename = "Award")]
    pub award: Award,
    #[serde(rename = "TakeContribution")]
    pub take_contribution: bool,
    #[serde(rename = "CanUncomplete")]
    pub can_uncomplete: bool,
    #[serde(rename = "IsInitialyActive")]
    pub is_initialy_active: bool,
    #[serde(rename = "IsInitialyVisible")]
    pub is_initialy_visible: bool,
    #[serde(rename = "IsHidden")]
    pub is_hidden: bool,
    #[serde(rename = "Ignore")]
    pub ignore: bool,
    #[serde(rename = "ShowCompleted")]
    pub show_completed: bool,
    #[serde(rename = "NeedComplete")]
    pub need_complete: bool,
    #[serde(rename = "StateChangeTrigger")]
    pub state_change_trigger: Trigger,
    #[serde(rename = "SoundActivated")]
    pub sound_activated: String,
    #[serde(rename = "SoundComplete")]
    pub sound_complete: String,
    #[serde(rename = "SoundFailed")]
    pub sound_failed: String,
    #[serde(rename = "AllowMultipleActivations")]
    pub allow_multiple_activations: bool,
    #[serde(rename = "AllowMultipleCompletions")]
    pub allow_multiple_completions: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Objectives {
    #[serde(rename = "Item")]
    pub items: Option<Vec<Quest>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestList {
    #[serde(rename = "Objectives")]
    pub objectives: Option<Objectives>,
    #[serde(rename = "DieInWeekWithoutTowns")]
    pub die_in_week_without_towns: bool
}