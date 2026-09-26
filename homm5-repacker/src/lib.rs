use std::{io::{Read, Write}, path::PathBuf};
use std::path::StripPrefixError;
use thiserror::Error;
use zip::{DateTime, write::SimpleFileOptions};
use zip::result::DateTimeRangeError;

#[derive(Debug, Error)]
pub enum RepackError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    DateTime(#[from] DateTimeRangeError),
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
    #[error(transparent)]
    StripPrefix(#[from] StripPrefixError),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
}

pub struct Repacker<'a> {
    pub from: &'a PathBuf,
    pub to: &'a PathBuf
}

impl<'a> Repacker<'a> {
    pub fn new(from: &'a PathBuf, to: &'a PathBuf) -> Self {
        Repacker {
            from,
            to
        }
    }

    pub fn run(&self) -> Result<(), RepackError> {
        let file = std::fs::File::create(self.to)?;
        let mut zipped_file = zip::ZipWriter::new(file);
        let file_options = SimpleFileOptions::default()
            .last_modified_time(DateTime::from_date_and_time(2107, 12, 31, 23, 59, 59)?);
        for entry in walkdir::WalkDir::new(self.from) {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() && !path.to_str().unwrap().contains(".git") {
                        let file_name = path.strip_prefix(self.from)?;
                        let mut curr_file = std::fs::File::open(path)?;
                        let mut container = Vec::new();
                        curr_file.read_to_end(&mut container)?;
                        zipped_file.start_file(file_name.to_str().unwrap(), file_options)?;
                        zipped_file.write_all(container.as_slice())?;
                    }
                },
                Err(e) => {
                   return Err(RepackError::WalkDir(e))
                }
            }
        }

        Ok(())
    }
}