use google_sheets4::api::ValueRange;

use crate::error::Error;

pub trait FromSheetValueRange
{
    type Output;

    fn convert_from_value_range(&self, values: ValueRange) -> Result<Self::Output, Error>;
}

pub trait IntoSheetsData<T>
{
    type Input;

    fn convert_into_sheets_data(&self, source: Self::Input) -> Result<T, Error>;
}