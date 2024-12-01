use std::{path::PathBuf, collections::HashMap};
use std::io::Read;

use rc_zip::{prelude::ReadZip};

const IGNORED_PARTS: [&'static str; 35] = [
    "_(Model)/", "Characters/", "Arenas/", "_(AIGeometry)/", "_(BasicSkelAnim)/", "_(AnimSet)/", "_(CameraSet)/", 
    "_(Decal)/", "_(DistanceFog)/", "_(Geometry)/", "_(HeightFog)/", 
    "_(Material)/", "_(Skeleton)/", "_(SunFlares)/", "ArenaObjects/", 
    "index.bin", "bin/", "Campaigns/", "Cameras/", "Custom/", 
    "Editor/", "_(Effect)/", "Lights/", "DialogScenes/", "RMG/", 
    "Scenes/", "scripts/", "Sounds/", "Roots/", ".bin", ".dds", ".ogg", ".tga", "types.xml", ".git"
];

#[derive(Debug, Clone)]
pub struct FileStructure {
    //pub key: String,
    pub pak: String,
    pub modified: i64,
    pub content: String
}

pub fn check_pak(path: PathBuf, files: &mut HashMap<String, FileStructure>) {
    let file = std::fs::File::open(&path).unwrap();
    let archive = file.read_zip().unwrap();
    for entry in archive.entries() {
        let name = entry.name().to_string();
        if (IGNORED_PARTS.iter().any(|part| entry.name().contains(part)) == false) && (entry.name().ends_with("/") == false) {
            if files.contains_key(entry.name().to_lowercase().as_str()) {
                if files.get(entry.name().to_lowercase().as_str()).unwrap().modified < entry.modified().timestamp() {
                    let mut content = String::new();
                    match entry.reader().read_to_string(&mut content) {
                        Ok(x) => {
                            files.insert(name.to_lowercase(), FileStructure { 
                                pak: path.to_str().unwrap().to_string(), 
                                modified: entry.modified().timestamp(),
                                content: content
                            });
                        }
                        Err(x) => {}
                    }
                }
            }
            else {
                let mut content = String::new();
                match entry.reader().read_to_string(&mut content) {
                    Ok(x) => {
                        files.insert(name.to_lowercase(), FileStructure { 
                            pak: path.to_str().unwrap().to_string(), 
                            modified: entry.modified().timestamp(),
                            content: content
                        });
                    }
                    Err(x) => {
                        //content = utf16_reader::read_to_string(entry.reader());
                        files.insert(name.to_lowercase(), FileStructure { 
                            pak: path.to_str().unwrap().to_string(), 
                            modified: entry.modified().timestamp(),
                            content: content
                        });
                    }
                }
            }
        }
    }
}