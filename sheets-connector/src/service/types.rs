use editor_tools::prelude::{ArmySlotStackCountGenerationMode, AssetArmySlotModel, AssetModel, DifficultyType};
use google_sheets4::api::ValueRange;
use serde::Deserialize;
use serde_json::{Number, Value};

use crate::{error::Error, utils::IntoSheetsData};

#[derive(Debug, Deserialize)]
pub(super) struct SheetCreationResponse {
    pub created_sheet_id: i32
}