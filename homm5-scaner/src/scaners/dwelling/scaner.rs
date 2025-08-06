use std::{collections::HashMap, str::FromStr};

use quick_xml::{events::Event, Reader};
use serde::{Deserialize, Serialize};

use crate::{core::Scan, error::ScanerError, pak::FileStructure, scaners::dwelling::model::{Dwelling, DwellingType}};

pub struct DwellingScaner;

#[derive(Debug, Serialize, Deserialize)]
pub struct DwellingScanerOutput {
    pub dwell_type: DwellingType,
    pub data: Dwelling
}

impl Scan for DwellingScaner {
    type Output = DwellingScanerOutput;
    fn scan(
        &mut self,
        file_key: &str,
        entity: &str,
        _files: &HashMap<String, FileStructure>,
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
                        break Ok(Some(DwellingScanerOutput { dwell_type: DwellingType::from_str(file_key).unwrap(), data}))
                    }
                }
                _ => (),
            }
            buf.clear();
        }
    }
}
