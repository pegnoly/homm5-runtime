use std::{path::PathBuf, io::{Read, Write}};

use zip::write::SimpleFileOptions;

pub struct Repacker {
    pub from: PathBuf,
    pub to: PathBuf
}

impl Repacker {
    pub fn new(from: PathBuf, to: PathBuf) -> Self {
        Repacker {
            from,
            to
        }
    }

    pub fn run(&self) {
        let file = std::fs::File::create(&self.to).unwrap();
        let mut zipped_file = zip::ZipWriter::new(file);
        for entry in walkdir::WalkDir::new(&self.from) {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    let name = entry.file_name().to_str().unwrap();
                    if path.is_file() && path.to_str().unwrap().contains(".git") == false {
                        let file_name = path.strip_prefix(&self.from).unwrap();
                        let mut curr_file = std::fs::File::open(&path).unwrap();
                        let mut container = Vec::new();
                        curr_file.read_to_end(&mut container).unwrap();
                        zipped_file.start_file(file_name.to_str().unwrap(), SimpleFileOptions::default()).unwrap();
                        zipped_file.write_all(&container.as_slice()).unwrap();
                    }
                },
                Err(error) => {}
            }
        }
    }
}