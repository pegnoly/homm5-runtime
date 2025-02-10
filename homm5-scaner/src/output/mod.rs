use std::path::PathBuf;

pub trait OutputJson {
    fn try_output(&self) -> String;
}