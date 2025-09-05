use serde::{Serialize, Serializer};
use std::{num::ParseIntError, str::Utf8Error, string::FromUtf8Error};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0} is not defined")]
    UndefinedData(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    Utf8(#[from] Utf8Error),
    #[error(transparent)]
    FromUft8(#[from] FromUtf8Error),
    #[error(transparent)]
    QuickXml(#[from] quick_xml::Error),
    #[error(transparent)]
    QuickXmlDE(#[from] quick_xml::DeError),
    #[error(transparent)]
    QuickXmlSE(#[from] quick_xml::SeError),
    #[error(transparent)]
    EditorTools(#[from] editor_tools::prelude::EditorToolsError),
    #[error(transparent)]
    Scaner(#[from] homm5_scaner::prelude::ScanerError),
    #[error(transparent)]
    SheetsConnector(#[from] sheets_connector::error::Error)
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
