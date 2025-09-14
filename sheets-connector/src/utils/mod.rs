use google_sheets4::api::{Request, ValueRange};

use crate::error::Error;

pub trait FromSheetValueRange
{
    type Output;

    fn from_value_range(&self, values: ValueRange) -> Result<Self::Output, Error>;
}

pub trait IntoSheetsData<T>
{
    type Input;

    fn into_sheets_data(&self, source: Self::Input) -> Result<T, Error>;
}