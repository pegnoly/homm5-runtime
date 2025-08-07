use std::{collections::HashMap, str::FromStr};

use quick_xml::{events::Event, Reader};
use serde::{Deserialize, Serialize};

use crate::{core::Scan, error::ScanerError, pak::FileStructure, scaners::dwelling::model::{Dwelling, DwellingType, Tile}, utils::configure_path};

pub struct DwellingScaner;

#[derive(Debug, Serialize, Deserialize)]
pub struct DwellingScanerOutput {
    pub dwell_type: DwellingType,
    pub data: DwellingLobbyData
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DwellingLobbyData {
    #[serde(rename = "Model")]
    pub model: Option<String>,
    #[serde(rename = "BlockedTiles")]
    pub blocked_tiles: Option<Vec<Tile>>,
    #[serde(rename = "ActiveTiles")]
    pub active_tiles: Option<Vec<Tile>>,
    #[serde(rename = "PossessionMarker")]
    pub possession_tile_marker: Option<Tile>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Desc")]
    pub desc: Option<String>,
    #[serde(rename = "Effect")]
    pub effect: Option<String>,
    #[serde(rename = "Icon")]
    pub icon: Option<String>
}

fn convert_to_lobby_data(dwelling: Dwelling, file_key: &str, files: &HashMap<String, FileStructure>) -> DwellingLobbyData {
    DwellingLobbyData {
        model: if let Some(file) = dwelling.model {
            file.href.map(|path| configure_path(Some(&path.replace("#xpointer(/Model)", "")), file_key, files))
        } else {
            None
        },
        active_tiles: if let Some(tiles) = dwelling.active_tiles {
            tiles.items
        } else {
            None
        },
        blocked_tiles: if let Some(tiles) = dwelling.blocked_tiles {
            tiles.items
        } else {
            None
        },
        possession_tile_marker: Some(dwelling.possession_tile_marker),
        name: if let Some(messages) = &dwelling.messages_file_ref {
            if let Some(items) = &messages.items {
                if let Some(data) = items.first() {
                    data.href.as_ref().map(|path| configure_path(Some(path), file_key, files))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        },
        desc: if let Some(messages) = dwelling.messages_file_ref {
            if let Some(items) = messages.items {
                if let Some(data) = items.get(1) {
                    data.href.as_ref().map(|path| configure_path(Some(path), file_key, files))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        },
        effect: if let Some(file) = dwelling.effect {
            file.href.map(|path| configure_path(Some(&path.replace("#xpointer(/Effect)", "")), file_key, files))
        } else {
            None
        },
        icon: if let Some(file) = dwelling.icon {
            file.href.map(|path| configure_path(Some(&path.replace("#xpointer(/Texture)", "")), file_key, files))
        } else {
            None
        },
    }
}

impl Scan for DwellingScaner {
    type Output = DwellingScanerOutput;
    fn scan(
        &mut self,
        file_key: &str,
        entity: &str,
        files: &HashMap<String, FileStructure>,
    ) -> Result<Option<Self::Output>, ScanerError> {
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(entity);
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break Ok(None),
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"AdvMapDwellingShared" {
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name()).unwrap().to_string();
                        let data: Dwelling = quick_xml::de::from_str(&format!("<AdvMapDwellingShared>{text}</AdvMapDwellingShared>")).unwrap();
                        break Ok(Some(DwellingScanerOutput { 
                            dwell_type: DwellingType::from_str(file_key).unwrap(), 
                            data: convert_to_lobby_data(data, file_key, files) 
                        }));
                    }
                }
                _ => (),
            }
            buf.clear();
        }
    }
}
