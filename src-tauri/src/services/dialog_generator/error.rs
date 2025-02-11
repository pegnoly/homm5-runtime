use derive_more::derive::{Display, From};

#[derive(From, Display, Debug)]
pub enum Error {
    SqlxError(sqlx::Error),
    JsonError(serde_json::Error),
    IOError(std::io::Error)
}