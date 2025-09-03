use std::str::Utf8Error;

#[derive(Debug, thiserror::Error)]
pub enum EditorToolsError {
    #[error(transparent)]
    SeaOrm(#[from] sea_orm::DbErr),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] Utf8Error),
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),
    #[error("Default editor tools error")]
    Default
}