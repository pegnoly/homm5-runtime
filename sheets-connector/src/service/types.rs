use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct SheetCreationResponse {
    pub created_sheet_id: i32,
}
