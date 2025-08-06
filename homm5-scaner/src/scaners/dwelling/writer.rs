use std::{collections::HashMap, fs::File, io::Write};
use serde::{Deserialize, Serialize};
use zip::ZipWriter;

use crate::{
    core::Output,
    error::ScanerError, scaners::dwelling::{model::{Dwelling, Tile}, scaner::DwellingScanerOutput},
};

pub struct DwellingDataOutput {
    entities: Vec<DwellingScanerOutput>,
}

impl DwellingDataOutput {
    pub fn new() -> Self {
        DwellingDataOutput {
            entities: vec![],
        }
    }
}

impl Default for DwellingDataOutput {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

impl From<Dwelling> for DwellingLobbyData {
    fn from(value: Dwelling) -> Self {
        DwellingLobbyData {
            model: if let Some(file) = value.model {
                file.href
            } else {
                None
            },
            active_tiles: if let Some(tiles) = value.active_tiles {
                tiles.items
            } else {
                None
            },
            blocked_tiles: if let Some(tiles) = value.blocked_tiles {
                tiles.items
            } else {
                None
            },
            possession_tile_marker: Some(value.possession_tile_marker),
            name: if let Some(messages) = &value.messages_file_ref {
                if let Some(items) = &messages.items {
                    if let Some(data) = items.first() {
                        data.href.clone()
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            },
            desc: if let Some(messages) = value.messages_file_ref {
                if let Some(items) = messages.items {
                    if let Some(data) = items.get(1) {
                        data.href.clone()
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            },
            effect: if let Some(file) = value.effect {
                file.href
            } else {
                None
            },
            icon: if let Some(file) = value.icon {
                file.href
            } else {
                None
            },
        }
    }
}

impl Output for DwellingDataOutput {
    type Input = DwellingScanerOutput;

    fn output_single(&mut self, object: Self::Input) -> Result<(), ScanerError> {
        self.entities.push(object);
        Ok(())
    }

    async fn finish_output(&self, _zip_writer: &mut ZipWriter<File>) -> Result<(), ScanerError> {
        let lobby_data: HashMap<super::model::DwellingType, DwellingLobbyData> = HashMap::from_iter(
            self.entities.iter().map(|dwell| {
                (dwell.dwell_type.clone(), DwellingLobbyData::from(dwell.data.clone()))
            })
        );
        let mut json_file = std::fs::File::create("D:\\dwellings.json")?;
        let json_string = serde_json::to_string_pretty(&lobby_data)?;
        json_file.write_all(json_string.as_bytes())?;

        Ok(())
    }
}
