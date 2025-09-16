use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CalamineCommon(#[from]calamine::Error),
    #[error(transparent)]
    CalamineXlsx(#[from]calamine::XlsxError),
    #[error(transparent)]
    IO(#[from]std::io::Error),
    #[error(transparent)]
    Sheets(#[from] Box<google_sheets4::Error>),
    #[error(transparent)]
    TokioTime(#[from]tokio::time::error::Elapsed),
    #[error(transparent)]
    Reqwest(#[from]reqwest::Error),
    #[error("{0} is not defined")]
    UndefinedValue(String),
    #[error(transparent)]
    ParseInt(#[from]std::num::ParseIntError)
}