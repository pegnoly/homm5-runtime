use crate::{
    Homm5Type,
    common::{ArmySlot, FileRef, SkillMastery, Trigger},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename = "cell")]
pub struct Cell {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Coordinates {
    #[serde(rename = "FloorID")]
    pub floor_id: u8,
    pub cell: Cell,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Target {
    #[serde(rename = "Type")]
    pub _type: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Coords")]
    pub coords: Coordinates,
}

impl Default for Target {
    fn default() -> Self {
        Target {
            _type: "ADV_TARGET_NONE".to_string(),
            name: None,
            coords: Coordinates::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TargetGlance {
    #[serde(rename = "Target")]
    pub target: Target,
    #[serde(rename = "Radius")]
    pub radius: u16,
    #[serde(rename = "Duration")]
    pub duration: u32,
}

impl Default for TargetGlance {
    fn default() -> Self {
        TargetGlance {
            target: Target::default(),
            radius: 10,
            duration: 5000,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
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
    pub gold: u16,
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

impl Default for Award {
    fn default() -> Self {
        Award {
            _type: "AWARD_NONE".to_string(),
            attribute: "HERO_ATTRIB_DEFENCE".to_string(),
            artifact_id: "ARTIFACT_NONE".to_string(),
            spell_id: "SPELL_NONE".to_string(),
            resources: Resource::default(),
            army_slot: ArmySlot::default(),
            skill_with_mastery: SkillMastery::default(),
            attribute_amount: 0,
            experience: 0,
            spell_points: 0,
            morale: 0,
            luck: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProgressFilesRefs {
    #[serde(rename = "Item")]
    pub items: Option<Vec<FileRef>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "Item")]
#[allow(non_snake_case)]
pub struct Quest {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "CaptionFileRef")]
    pub caption_file_ref: FileRef,
    #[serde(rename = "ObscureCaptionFileRef")]
    pub obscure_caption_file_ref: FileRef,
    #[serde(rename = "DescriptionFileRef")]
    pub description_file_ref: FileRef,
    #[serde(rename = "ProgressCommentsFileRef")]
    pub progress_comments_file_ref: ProgressFilesRefs,
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "Parameters")]
    pub parameters: Option<String>,
    #[serde(rename = "Timeout")]
    pub timeout: i8,
    #[serde(rename = "Holdout")]
    pub holdout: i8,
    #[serde(rename = "CheckDelay")]
    pub check_delay: i8,
    #[serde(rename = "Dependencies")]
    pub dependencies: Option<String>,
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
    pub sound_activated: Option<String>,
    #[serde(rename = "SoundComplete")]
    pub sound_complete: Option<String>,
    #[serde(rename = "SoundFailed")]
    pub sound_failed: Option<String>,
    #[serde(rename = "AllowMultipleActivations")]
    #[serde(default)]
    pub allow_multiple_activations: bool,
    #[serde(rename = "AllowMultipleCompletions")]
    #[serde(default)]
    pub allow_multiple_completions: bool,
}

impl Default for Quest {
    fn default() -> Self {
        Quest {
            name: Some(String::new()),
            kind: "OBJECTIVE_KIND_MANUAL".to_string(),
            caption_file_ref: FileRef { href: None },
            obscure_caption_file_ref: FileRef { href: None },
            description_file_ref: FileRef { href: None },
            progress_comments_file_ref: ProgressFilesRefs::default(),
            parameters: None,
            timeout: -1,
            holdout: -1,
            check_delay: -1,
            dependencies: None,
            instant_victory: false,
            target_glance: TargetGlance::default(),
            award: Award::default(),
            take_contribution: false,
            can_uncomplete: false,
            is_initialy_active: false,
            is_initialy_visible: false,
            is_hidden: false,
            ignore: false,
            show_completed: false,
            need_complete: false,
            state_change_trigger: Trigger::default(),
            sound_activated: None,
            sound_complete: None,
            sound_failed: None,
            allow_multiple_activations: false,
            allow_multiple_completions: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Objectives {
    #[serde(rename = "Item")]
    pub items: Option<Vec<Quest>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestList {
    #[serde(rename = "Objectives")]
    pub objectives: Option<Objectives>,
    #[serde(rename = "DieInWeekWithoutTowns")]
    pub die_in_week_without_towns: bool,
}

impl Homm5Type for Quest {}
