use thiserror::Error;

#[derive(Error, Debug)]
pub enum  MapModifierError {
    #[error(transparent)]
    IO(#[from]std::io::Error),
    #[error(transparent)]
    Bincode(#[from] bincode::Error),
}