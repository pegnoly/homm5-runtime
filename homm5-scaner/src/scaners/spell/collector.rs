use crate::{
    core::{CollectFiles, FileObjects},
    error::ScanerError,
    pak::FileStructure,
};
use quick_xml::{Reader, events::Event};
use std::collections::HashMap;

pub struct SpellFileCollector;

impl CollectFiles for SpellFileCollector {
    fn collect(
        &self,
        files: &HashMap<String, FileStructure>,
        collected_files: &mut Vec<(String, FileStructure)>,
    ) -> Result<(), ScanerError> {
        let spells_xdb = files
            .iter()
            .find(|f| {
                f.0 == "GameMechanics/RefTables/UndividedSpells.xdb"
                    .to_lowercase()
                    .as_str()
            })
            .unwrap();
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(spells_xdb.1.content.as_str());
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
                        let spells_de: Result<FileObjects, quick_xml::DeError> =
                            quick_xml::de::from_str(&text);
                        match spells_de {
                            Ok(spells) => {
                                for spell in spells.objects {
                                    if let Some(obj) = spell.Obj
                                        && let Some(href) = obj.href.as_ref() {
                                            let spell_key = href
                                                .replace("#xpointer(/Spell)", "")
                                                .trim_start_matches("/")
                                                .to_lowercase();
                                            let spell_entity = files.get(&spell_key);
                                            match spell_entity {
                                                Some(entity) => {
                                                    let mut entity_cloned = entity.clone();
                                                    entity_cloned.id = Some(spell.ID);
                                                    collected_files
                                                        .push((spell_key.clone(), entity_cloned));
                                                }
                                                None => {
                                                    println!("Key {} is not in files", &spell_key)
                                                }
                                            }
                                        }
                                }
                            }
                            Err(e) => println!("Error deserializing spells.xdb, {e}"),
                        }
                    }
                }
                _ => (),
            }
            buf.clear();
        }
    }
}
