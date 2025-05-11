use std::collections::HashMap;

use homm5_types::{common::FileRef, spell::SpellShared};
use quick_xml::{events::Event, Reader};

use crate::{core::Scan, error::ScanerError, pak::FileStructure, utils::configure_path};

use super::model;

pub struct SpellScaner {
    pub id: i32,
}

impl Scan for SpellScaner {
    type Output = model::Model;

    fn scan(
        &mut self,
        file_key: &String,
        entity: &String,
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
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"Spell" => {
                        let end = e.to_end().into_owned();
                        let possible_text = reader.read_text(end.name());
                        match possible_text {
                            Ok(text) => {
                                let text = text.to_string();
                                let de_res: Result<SpellShared, quick_xml::DeError> =
                                    quick_xml::de::from_str(&format!("<Spell>{}</Spell>", text));
                                match de_res {
                                    Ok(mut spell) => {
                                        let name = configure_path(
                                            spell.NameFileRef.as_ref().unwrap().href.as_ref(),
                                            file_key,
                                            files,
                                        );
                                        let desc = configure_path(
                                            spell
                                                .LongDescriptionFileRef
                                                .as_ref()
                                                .unwrap()
                                                .href
                                                .as_ref(),
                                            file_key,
                                            files,
                                        );
                                        let icon_key = spell
                                            .Texture
                                            .as_ref()
                                            .unwrap()
                                            .href
                                            .as_ref()
                                            .unwrap_or(&String::new())
                                            .replace("#xpointer(/Texture)", "")
                                            .to_lowercase();
                                        let icon = configure_path(Some(&icon_key), file_key, files);
                                        spell.NameFileRef = Some(FileRef { href: Some(name) });
                                        spell.LongDescriptionFileRef =
                                            Some(FileRef { href: Some(desc) });
                                        spell.Texture = Some(FileRef { href: Some(icon) });
                                        self.id += 1;

                                        let mut db_model = model::Model::from(spell);
                                        db_model.id = self.id;
                                        if !db_model.name_txt.is_empty() {
                                            if let Some(data) = files.get(&db_model.name_txt) {
                                                db_model.name = data.content.clone();
                                            }
                                        }
                                        if !db_model.desc_txt.is_empty() {
                                            if let Some(data) = files.get(&db_model.desc_txt) {
                                                db_model.desc = data.content.clone();
                                            }
                                        }
                                        break Ok(Some(db_model));
                                    }
                                    Err(e) => {
                                        println!(
                                            "error while deserializing file key {}, {:?}",
                                            file_key,
                                            e.to_string()
                                        );
                                    }
                                }
                            }
                            Err(_e) => println!("error reading file content: {}", file_key),
                        }
                    }
                    _ => {}
                },
                _ => (),
            }
            buf.clear();
        }
    }
}