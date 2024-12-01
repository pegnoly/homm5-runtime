use serde::{Serialize, Deserialize};

/// Common types suitable for any others.

/// Position of object on the map
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Reference to file(<Shared href="some_path"/>)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileRef {
    #[serde(rename = "@href")]
    pub href: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    #[serde(rename = "FunctionName")]
    pub function_name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trigger {
    #[serde(rename = "Action")]
    pub action: Action
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArmySlot {
    #[serde(rename = "Creature")]
    pub creature: String,
    #[serde(rename = "Count")]
    pub count: u16
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkillMastery {
    #[serde(rename = "Mastery")]
    pub mastery: String,
    #[serde(rename = "SkillID")]
    pub skill_id: String
}