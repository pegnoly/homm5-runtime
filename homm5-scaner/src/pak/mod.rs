use core::str;
use std::io::Read;
use std::{collections::HashMap, path::PathBuf};

use bincode::config::Config;
use rc_zip::prelude::ReadZip;
use utf16string::WStr;

pub(crate) const EXTENSIONS: [&str; 4] = [".pak", ".h5m", ".h5c", ".h5u"];

const IGNORED_PARTS: [&'static str; 35] = [
    "_(Model)/",
    "Characters/",
    "Arenas/",
    "_(AIGeometry)/",
    "_(BasicSkelAnim)/",
    "_(AnimSet)/",
    "_(CameraSet)/",
    "_(Decal)/",
    "_(DistanceFog)/",
    "_(Geometry)/",
    "_(HeightFog)/",
    "_(Material)/",
    "_(Skeleton)/",
    "_(SunFlares)/",
    "ArenaObjects/",
    "index.bin",
    "bin/",
    "Campaigns/",
    "Cameras/",
    "Custom/",
    "Editor/",
    "_(Effect)/",
    "Lights/",
    "DialogScenes/",
    "RMG/",
    "Scenes/",
    "scripts/",
    "Sounds/",
    "Roots/",
    ".bin",
    ".dds",
    ".ogg",
    ".tga",
    "types.xml",
    ".git",
];

#[derive(Debug, Clone)]
pub struct FileStructure {
    //pub key: String,
    pub pak: String,
    pub modified: i64,
    pub content: String,
}

pub fn check_pak(path: PathBuf, files: &mut HashMap<String, FileStructure>) {
    println!("Reading {:?}", &path);
    let file = std::fs::File::open(&path).unwrap();
    let archive = file.read_zip().unwrap();
    for entry in archive.entries() {
        let name = entry.name().to_string().replace("\\", "/");
        if (IGNORED_PARTS.iter().any(|part| name.contains(part)) == false)
            && (name.ends_with("/") == false)
        {
            let name = name.to_lowercase();
            if files.contains_key(&name) {
                //println!("Already written file {} found in {:?}", &name, &path);
                if files.get(&name).unwrap().modified < entry.modified().timestamp() {
                    //println!("Newer version of file {} found in {:?}", &name, &path);
                    if name.ends_with(".txt") {
                        //println!("Inserting without reading {}", &name.to_lowercase());
                        let mut buff = vec![];
                        match entry.reader().read_to_end(&mut buff) {
                            Ok(_x) => {
                                if let Ok(utf16string) = WStr::from_utf16le(&buff) {
                                    files.insert(
                                        name.to_lowercase().replace("\\", "/"),
                                        FileStructure {
                                            pak: path.to_str().unwrap().to_string(),
                                            modified: entry.modified().timestamp(),
                                            content: utf16string.to_string(),
                                        },
                                    );
                                } else {
                                    files.insert(
                                        name.to_lowercase().replace("\\", "/"),
                                        FileStructure {
                                            pak: path.to_str().unwrap().to_string(),
                                            modified: entry.modified().timestamp(),
                                            content: String::new(),
                                        },
                                    );
                                }
                            },
                            Err(_error) => {
                                files.insert(
                                    name.to_lowercase().replace("\\", "/"),
                                    FileStructure {
                                        pak: path.to_str().unwrap().to_string(),
                                        modified: entry.modified().timestamp(),
                                        content: String::new(),
                                    },
                                );
                            }
                        }
                    } else {
                        let mut content = String::new();
                        match entry.reader().read_to_string(&mut content) {
                            Ok(_x) => {
                                files.insert(
                                    name.to_lowercase(),
                                    FileStructure {
                                        pak: path.to_str().unwrap().to_string(),
                                        modified: entry.modified().timestamp(),
                                        content: content,
                                    },
                                );
                            }
                            Err(_x) => {
                                files.insert(
                                    name.to_lowercase(),
                                    FileStructure {
                                        pak: path.to_str().unwrap().to_string(),
                                        modified: entry.modified().timestamp(),
                                        content: content,
                                    },
                                );
                            }
                        }
                    }
                }
            } else {
                if name.ends_with(".txt") {
                    let mut buff = vec![];
                    match entry.reader().read_to_end(&mut buff) {
                        Ok(_x) => {
                            if let Ok(utf16string) = WStr::from_utf16le(&buff) {
                                files.insert(
                                    name.to_lowercase().replace("\\", "/"),
                                    FileStructure {
                                        pak: path.to_str().unwrap().to_string(),
                                        modified: entry.modified().timestamp(),
                                        content: utf16string.to_string(),
                                    },
                                );
                            } else {
                                files.insert(
                                    name.to_lowercase().replace("\\", "/"),
                                    FileStructure {
                                        pak: path.to_str().unwrap().to_string(),
                                        modified: entry.modified().timestamp(),
                                        content: String::new(),
                                    },
                                );
                            }
                        },
                        Err(_error) => {
                            files.insert(
                                name.to_lowercase().replace("\\", "/"),
                                FileStructure {
                                    pak: path.to_str().unwrap().to_string(),
                                    modified: entry.modified().timestamp(),
                                    content: String::new(),
                                },
                            );
                        }
                    }
                } else {
                    let mut content = String::new();
                    match entry.reader().read_to_string(&mut content) {
                        Ok(_x) => {
                            files.insert(
                                name.to_lowercase(),
                                FileStructure {
                                    pak: path.to_str().unwrap().to_string(),
                                    modified: entry.modified().timestamp(),
                                    content: content,
                                },
                            );
                        }
                        Err(_x) => {
                            files.insert(
                                name.to_lowercase(),
                                FileStructure {
                                    pak: path.to_str().unwrap().to_string(),
                                    modified: entry.modified().timestamp(),
                                    content: content,
                                },
                            );
                        }
                    }
                }
            }
        }
    }
}
