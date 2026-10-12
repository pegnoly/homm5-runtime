use google_sheets4::api::ValueRange;
use crate::error::Error;

pub trait SheetsConvertable {}

impl SheetsConvertable for ValueRange {}
impl SheetsConvertable for Vec<ValueRange> {}

pub trait FromSheetValueRange<T: SheetsConvertable> {
    type Output;

    fn convert_from_sheets_data(&self, values: T) -> Result<Self::Output, Error>;
}

pub trait IntoSheetsData<T: SheetsConvertable> {
    type Input;

    fn convert_into_sheets_data(&self, source: Self::Input) -> Result<T, Error>;
}
