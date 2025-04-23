#[derive(Debug, thiserror::Error)]
pub enum EditorToolsError {
    #[error(transparent)]
    SeaOrm(#[from]sea_orm::DbErr)
}