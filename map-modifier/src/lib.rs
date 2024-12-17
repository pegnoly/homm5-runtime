use std::path::PathBuf;

use homm5_types::{quest::Quest, Homm5Type};
use serde::{Deserialize, Serialize};

pub mod quest;

pub trait GenerateBoilerplate {
    type Output: Homm5Type;

    fn generate(&self) -> Self::Output;
}

pub struct ModifiersQueue {
    quest_queue: Vec<Box<dyn GenerateBoilerplate<Output = Quest>>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub id: u16,
    pub name: String,
    pub campaign: u8,
    pub mission: u8,
    pub xdb: String,
    pub data_path: String
}

