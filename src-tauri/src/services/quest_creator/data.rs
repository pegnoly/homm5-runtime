use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QuestProgressDBModel {
    pub id: Uuid,
    pub quest_id: Uuid,
    pub number: u32,
    pub text: String,
    pub concatenate: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestProgressFrontendModel {
    pub text: String,
    pub concatenate: bool
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QuestDBModel {
    pub id: Uuid,
    pub directory: String,
    pub campaign_number: u32,
    pub mission_number: u32,
    pub name: String,
    pub desc: String,
    pub script_name: String,
    pub is_active: bool,
    pub is_secondary: bool,
    pub is_first_init: bool
}

// full form of quest frontend needs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestFrontendModel {
    pub id: Uuid,
    pub directory: String,
    pub name: String,
    pub desc: String,
    pub script_name: String
}

// form of quest frontend needs to setup selection
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestLoadingModel {
    pub id: Uuid,
    pub name: String
}

impl From<QuestDBModel> for QuestFrontendModel {
    fn from(value: QuestDBModel) -> Self {
        QuestFrontendModel { 
            id: value.id, 
            directory: value.directory, 
            name: value.name, 
            desc: value.desc, 
            script_name: value.script_name 
        }
    }
}

impl From<QuestDBModel> for QuestLoadingModel {
    fn from(value: QuestDBModel) -> Self {
        QuestLoadingModel { id: value.id, name: value.name }
    }
}

impl From<QuestProgressDBModel> for QuestProgressFrontendModel {
    fn from(value: QuestProgressDBModel) -> Self {
        QuestProgressFrontendModel { text: value.text, concatenate: value.concatenate }
    }
}