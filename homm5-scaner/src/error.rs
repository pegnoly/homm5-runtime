use zip::result::ZipError;

#[derive(Debug, thiserror::Error)]
pub enum ScanerError {
    #[error(transparent)]
    Xml(#[from]quick_xml::Error),
    #[error(transparent)]
    XmlDe(#[from]quick_xml::DeError),
    #[error(transparent)]
    Db(#[from]sea_orm::DbErr),
    #[error(transparent)]
    Zip(#[from] ZipError),
    #[error(transparent)]
    IO(#[from] std::io::Error)
}