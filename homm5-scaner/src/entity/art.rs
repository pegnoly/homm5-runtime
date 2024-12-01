use serde::{Serialize, Deserialize};
use super::{Scan, Output, FileStructure, CollectFiles};
use quick_xml::{Reader, events::Event};
use std::collections::HashMap;
use homm5_types::art::AdvMapArtifactShared;

impl Output for AdvMapArtifactShared {
    type ID = u16;

    fn to_lua(&self, id: Option<Self::ID>) -> String {
        let is_sellable = if self.CanBeGeneratedToSell == true {"1"} else {"nil"};
        format!(
            "\t[{}] = {{
        is_sellable = {},
        name = \"{}\",
        desc = \"{}\",
        icon = \"{}\",
        cost = {},
        slot = {},
        type = {}
    }},\n", 
            id.unwrap() - 1,
            is_sellable,
            self.NameFileRef.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()), 
            self.DescriptionFileRef.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()),
            self.Icon.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()),
            self.CostOfGold,
            self.Slot,
            self.Type
        )
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ArtObject {
    pub ID: String,
    pub obj: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtObjects {
    #[serde(rename = "Item")]
    pub arts: Vec<ArtObject>
}

pub struct ArtFileCollector {}

impl CollectFiles for ArtFileCollector {
    fn collect(&self, files: &HashMap<String, FileStructure>, collected_files: &mut Vec<(String, FileStructure)>) {
        let arts_xdb = files.iter()
            .find(|f| f.0 == "GameMechanics/RefTables/Artifacts.xdb".to_lowercase().as_str())
            .unwrap();
        //collected_files.push((arts_xdb.0.clone(), arts_xdb.1.clone()));
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(arts_xdb.1.content.as_str());
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"obj" => {
                            let end = e.to_end().into_owned();
                            let text = reader.read_text(end.name()).unwrap().to_string();
                            let text = format!("<obj>{}</obj>", text);
                            collected_files.push(("GameMechanics/RefTables/Artifacts.xdb".to_lowercase(), FileStructure{
                                pak: arts_xdb.1.pak.clone(),
                                modified: arts_xdb.1.modified,
                                content: text
                            }));
                        }
                        _=> {}
                    }
                }
                _ => ()
            }
            buf.clear();
        }
    }
}

pub struct ArtScaner {
    pub id: u16,
}

impl Scan<u16> for ArtScaner {
    fn get_id(&self) -> Option<u16> {
        Some(self.id)
    }

    #[allow(unused_variables)]
    fn scan(&mut self, file_key: &String, entity: &String, files: &HashMap<String, FileStructure>) -> Option<Box<dyn Output<ID = u16>>> {
        let art_de: Result<AdvMapArtifactShared, quick_xml::DeError> = quick_xml::de::from_str(entity);
        match art_de {
            Ok(art) => {
                self.id += 1;
                Some(Box::new(art))
            }
            Err(e) => {
                println!("error deserializing artifact {}", e.to_string());
                None
            }
        }
    }
}