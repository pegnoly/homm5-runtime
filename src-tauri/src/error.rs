use derive_more::derive::{Display, From};
use serde::{Serialize, Serializer};

#[derive(From, Display)]
pub enum Error {
    SqlxError(sqlx::Error),
    JsonError(serde_json::Error),
    IOError(std::io::Error)
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer 
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}