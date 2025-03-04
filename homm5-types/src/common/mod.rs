use serde::{Deserialize, Serialize};

/// Common types suitable for any others.

/// Position of object on the map
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Reference to file(<Shared href="some_path"/>)
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FileRef {
    #[serde(rename = "@href")]
    pub href: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct XmlArray<T: Serialize + Clone> {
    #[serde(rename = "Item")]
    pub items: Vec<Option<T>>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Action {
    #[serde(rename = "FunctionName")]
    pub function_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Trigger {
    #[serde(rename = "Action")]
    pub action: Action,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArmySlot {
    #[serde(rename = "Creature")]
    pub creature: String,
    #[serde(rename = "Count")]
    pub count: u16,
}

impl Default for ArmySlot {
    fn default() -> Self {
        ArmySlot {
            creature: "CREATURE_UNKNOWN".to_string(),
            count: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkillMastery {
    #[serde(rename = "Mastery")]
    pub mastery: String,
    #[serde(rename = "SkillID")]
    pub skill_id: String,
}

impl Default for SkillMastery {
    fn default() -> Self {
        SkillMastery {
            mastery: "MASTERY_NONE".to_string(),
            skill_id: "HERO_SKILL_NONE".to_string(),
        }
    }
}
