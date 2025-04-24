use std::num::ParseIntError;

use derive_more::derive::{Display, From};
use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from]sqlx::Error),
    #[error(transparent)]
    Json(#[from]serde_json::Error),
    #[error(transparent)]
    IO(#[from]std::io::Error),
    #[error(transparent)]
    ParseInt(#[from]ParseIntError),
    #[error(transparent)]
    EditorTools(#[from]editor_tools::error::EditorToolsError)
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
