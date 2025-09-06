use google_sheets4::api::ValueRange;

use crate::error::Error;

pub trait SheetsValueRangeConverter
{
    type Output;

    fn convert(&self, values: ValueRange) -> Result<Self::Output, Error>;
}