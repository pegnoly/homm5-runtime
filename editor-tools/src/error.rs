use std::str::Utf8Error;

#[derive(Debug, thiserror::Error)]
pub enum EditorToolsError {
    #[error(transparent)]
    SeaOrm(#[from] sea_orm::DbErr),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error(transparent)]
    Resize(#[from] resize::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Dds(#[from] ddsfile::Error),
    #[error(transparent)]
    Utf8(#[from] Utf8Error),
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),
    #[error(transparent)]
    Encode(#[from] bincode::error::EncodeError),
    #[error("Default editor tools error")]
    Default
}
