use derive_more::derive::{Display, From};

#[derive(From, Display)]
pub enum Error {
    SqlxError(sqlx::Error),
    JsonError(serde_json::Error)
}