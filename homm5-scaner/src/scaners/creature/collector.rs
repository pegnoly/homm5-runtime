use std::collections::HashMap;

use quick_xml::{Reader, events::Event};

use crate::{
    core::{CollectFiles, FileObjects},
    error::ScanerError,
    pak::FileStructure,
};

pub struct CreatureFilesCollector;

impl CollectFiles for CreatureFilesCollector {
    fn collect(
        &self,
        files: &HashMap<String, FileStructure>,
        collected_files: &mut Vec<(String, FileStructure)>,
    ) -> Result<(), ScanerError> {
        let creatures_xdb = files
            .iter()
            .find(|f| {
                f.0 == "GameMechanics/RefTables/Creatures.xdb"
                    .to_lowercase()
                    .as_str()
            })
            .unwrap();
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(creatures_xdb.1.content.as_str());
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break Ok(()),
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"objects" {
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name()).unwrap().to_string();
                        let text = format!("<objects>{text}</objects>");
                        let creatures_de: Result<FileObjects, quick_xml::DeError> =
                            quick_xml::de::from_str(&text);
                        match creatures_de {
                            Ok(creatures) => {
                                for creature in creatures.objects {
                                    if let Some(obj) = creature.Obj {
                                        let creature_key = obj
                                            .href
                                            .as_ref()
                                            .unwrap()
                                            .replace("#xpointer(/Creature)", "")
                                            .trim_start_matches("/")
                                            .to_lowercase();
                                        let creature_entity = files.get(&creature_key);
                                        match creature_entity {
                                            Some(entity) => {
                                                collected_files
                                                    .push((creature_key.clone(), entity.clone()));
                                            }
                                            None => {
                                                println!("Key {} is not in files", &creature_key)
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error deserializing creatures.xdb, {e}")
                            }
                        }
                    }
                }
                _ => (),
            }
            buf.clear();
        }
    }
}
