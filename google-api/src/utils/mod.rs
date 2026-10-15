use google_sheets4::api::ValueRange;
use itertools::Itertools;
use crate::error::Error;
use crate::error::Error::ConversionError;

pub trait FromSheetsData<T: TryFrom<ValueRange>> {
    #[allow(clippy::wrong_self_convention)]
    fn from_single(&self, value: ValueRange) -> Result<T, Error> {
        T::try_from(value).map_err(|e| ConversionError("ValueRange".to_string()))
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_many(&self, value: Vec<ValueRange>) -> Result<Vec<T>, Error> {
        value.into_iter().map(|v| T::try_from(v)).collect::<Result<Vec<_>, _>>()
            .map_err(|e| ConversionError("Vec<ValueRange>".to_string()))
    }
}

pub trait IntoSheetsData<T: Into<ValueRange>> {
    #[allow(clippy::wrong_self_convention)]
    fn into_single(&self, source: T) -> Result<ValueRange, Error> {
        Ok(source.into())
    }

    #[allow(clippy::wrong_self_convention)]
    fn into_many(&self, source: Vec<T>) -> Result<Vec<ValueRange>, Error> {
        Ok(source.into_iter().map(Into::into).collect())
    }
}
