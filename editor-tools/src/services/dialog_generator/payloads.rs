use uuid::Uuid;
use crate::services::dialog_generator::models::speaker::SpeakerType;

pub struct CreateDialogPayload {
    pub mission_id: i32,
    pub name: String,
    pub script_name: String,
    pub directory: String,
    pub speakers: Vec<Uuid>,
}

pub struct CreateSpeakerPayload {
    pub name: String,
    pub script_name: String,
    pub color: String,
    pub speaker_type: SpeakerType,
    pub local_to_map: Option<i32>
}

#[derive(Debug)]
pub struct GetDialogVariantPayload {
    pub dialog_id: Uuid,
    pub step: i32,
    pub label: String,
}

pub struct UpdateLabelsPayload {
    pub dialog_id: Uuid,
    pub labels: Vec<String>,
}

pub struct CreateDialogVariantPayload {
    pub dialog_id: Uuid,
    pub step: i32,
    pub label: String,
}

pub struct SaveVariantPayload {
    pub id: Uuid,
    pub text: String,
    pub speakers: Vec<Uuid>,
}
