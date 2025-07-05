use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatableCreatureModel {
    pub id: i32,
    pub base_creature: Option<i32>,
    pub inner_name: Option<String>,
    pub parent_creature: Option<i32>,
    pub upgrades: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureGeneratorSessionConfig {
    pub name: String,
    pub current_id: i32,
    pub created_ids: Vec<i32>,
    pub models: Vec<CreatableCreatureModel>,
    pub selected_abilities: Vec<i32>
}