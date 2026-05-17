use sea_orm::FromJsonQueryResult;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "quest_progresses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub quest_id: i32,
    pub number: i32,
    pub text: Option<String>,
    pub one_of_progress: Option<OneOfQuestProgress>,
    pub concatenate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromJsonQueryResult)]
pub struct OneOfQuestProgress {
    pub text: String,
    pub count: i32,
    pub start_value: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum QuestProgressType {
    Default(String),
    OneOf(OneOfQuestProgress)
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
