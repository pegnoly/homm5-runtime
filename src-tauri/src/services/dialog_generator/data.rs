use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::Type)]
#[repr(i32)]
pub enum SpeakerType {
    //#[strum(to_string="SPEAKER_TYPE_HERO")]
    NoSpeaker,
    Hero,
    //#[strum(to_string="SPEAKER_TYPE_CREATURE")]
    Creature
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct SpeakerDBModel {
    pub id: Uuid,
    pub name: String,
    pub script_name: String,
    pub color: String,
    pub speaker_type: SpeakerType
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpeakerFrontendModel {
    pub id: Uuid,
    pub name: String
}

impl From<SpeakerDBModel> for SpeakerFrontendModel {
    fn from(value: SpeakerDBModel) -> Self {
        SpeakerFrontendModel { id: value.id, name: value.name }
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct DialogDBModel {
    pub id: Uuid,
    pub name: String,
    pub script_name: String,
    pub directory: String,
    pub speakers_ids: String,
    pub labels: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DialogFrontendModel {
    pub id: Uuid,
    pub name: String,
}

impl From<DialogDBModel> for DialogFrontendModel {
    fn from(value: DialogDBModel) -> Self {
        DialogFrontendModel { id: value.id, name: value.name }
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct DialogVariantModel {
    pub id: Uuid,
    pub dialog_id: Uuid,
    pub step: u32,
    pub label: String,
    pub speaker_id: Uuid,
    pub text: String
}