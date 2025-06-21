use crate::services::dialog_generator::models::speaker::SpeakerType;

pub struct CreateDialogPayload {
    pub mission_id: i32,
    pub name: String,
    pub script_name: String,
    pub directory: String,
    pub speakers: Vec<i32>
}

pub struct CreateSpeakerPayload {
    pub name: String,
    pub script_name: String,
    pub color: String,
    pub speaker_type: SpeakerType
}

pub struct GetVariantPayload {
    pub dialog_id: i32,
    pub step: i32,
    pub label: String
}

pub struct UpdateLabelsPayload {
    pub dialog_id: i32,
    pub labels: Vec<String>
}

pub struct CreateVariantPayload {
    pub dialog_id: i32,
    pub step: i32,
    pub label: String
}

pub struct SaveVariantPayload {
    pub id: i32,
    pub text: String,
    pub speaker: i32
}