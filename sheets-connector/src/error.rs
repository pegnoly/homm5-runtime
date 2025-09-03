use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CalamineCommon(#[from]calamine::Error),
    #[error(transparent)]
    CalamineXlsx(#[from]calamine::XlsxError),
    #[error(transparent)]
    IO(#[from]std::io::Error)
}