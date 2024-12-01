use super::{configure_path, CollectFiles, FileObjects, FileStructure, Output, Scan};
use quick_xml::{Reader, events::Event};
use std::collections::HashMap;
use homm5_types::{common::FileRef, spell::SpellShared};

impl Output for SpellShared {
    type ID = u16;
    fn to_lua(&self, id: Option<Self::ID>) -> String {
        let is_aimed = if self.IsAimed == true {"1"} else {"nil"};
        let is_area = if self.IsAreaAttack == true {"1"} else {"nil"};
        format!(
            "\t[{}] = {{
        name = \"{}\",
        desc = \"{}\",
        icon = \"{}\",
        cost = {},
        school = {},
        level = {},
        is_aimed = {},
        is_area = {}
    }},\n", 
            id.unwrap(),
            self.NameFileRef.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()), 
            self.LongDescriptionFileRef.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()),
            self.Texture.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()),
            self.TrainedCost,
            self.MagicSchool,
            self.Level,
            is_aimed,
            is_area
        )
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

pub struct SpellFileCollector { 
}

impl CollectFiles for SpellFileCollector {
    fn collect(&self, files: &HashMap<String, FileStructure>, collected_files: &mut Vec<(String, FileStructure)>) {
        let spells_xdb = files.iter()
        .find(|f| f.0 == "GameMechanics/RefTables/UndividedSpells.xdb".to_lowercase().as_str())
        .unwrap();
    let mut buf = Vec::new();
    let mut reader = Reader::from_str(spells_xdb.1.content.as_str());
    reader.trim_text(true);
    reader.expand_empty_elements(true);
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"objects" => {
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name()).unwrap().to_string();
                        let text = format!("<objects>{}</objects>", text);
                        let spells_de: Result<FileObjects, quick_xml::DeError> = quick_xml::de::from_str(&text);
                        match spells_de {
                            Ok(spells) => {
                                for spell in spells.objects {
                                    match spell.Obj {
                                        Some(obj) => {
                                            match obj.href.as_ref() {
                                                Some(href) => {
                                                    let spell_key = href
                                                        .replace("#xpointer(/Spell)", "")
                                                        .trim_start_matches("/")
                                                        .to_lowercase();
                                                    let spell_entity = files.get(&spell_key);
                                                    match spell_entity {
                                                        Some(entity) => {
                                                            collected_files.push((spell_key.clone(), entity.clone()));
                                                        },
                                                        None => println!("Key {} is not in files", &spell_key)
                                                    }
                                                },
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            },
                            Err(e) => println!("Error deserializing spells.xdb, {}", e.to_string())
                        }
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

pub struct SpellScaner {
    pub id: u16
}

impl Scan<u16> for SpellScaner {
    fn scan(&mut self, file_key: &String, entity: &String, files: &HashMap<String, FileStructure>) -> Option<Box<dyn Output<ID = u16>>> {
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(entity);
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break None,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"Spell" => {
                            let end = e.to_end().into_owned();
                            let possible_text = reader.read_text(end.name());
                            match possible_text {
                                Ok(text) => {
                                    let text = text.to_string();
                                    let de_res: Result<SpellShared, quick_xml::DeError> = quick_xml::de::from_str(
                                        &format!("<Spell>{}</Spell>", text)
                                    );
                                    match de_res {
                                        Ok(mut spell) => {
                                            let name = configure_path(spell.NameFileRef.as_ref().unwrap().href.as_ref(), file_key, files);
                                            let desc = configure_path(spell.LongDescriptionFileRef.as_ref().unwrap().href.as_ref(), file_key, files);
                                            let icon_key = spell.Texture.as_ref().unwrap().href.as_ref().unwrap_or(&String::new())
                                                .replace("#xpointer(/Texture)", "")
                                                .to_lowercase();
                                            let icon = configure_path(Some(&icon_key), file_key, files);
                                            spell.NameFileRef = Some(FileRef { href: Some(name) });
                                            spell.LongDescriptionFileRef = Some(FileRef { href: Some(desc) });
                                            spell.Texture = Some(FileRef { href: Some(icon) });
                                            self.id+=1;
                                            break Some(Box::new(spell));
                                        }
                                        Err(e) => {
                                            println!("error while deserializing file key {}, {:?}", file_key, e.to_string());
                                        }
                                    }
                                },
                                Err(_e) => println!("error reading file content: {}", file_key)
                            }
                        }
                        _=> {}
                    }
                }
                _ => ()
            }
            buf.clear();
        }
    }

    fn get_id(&self) -> Option<u16> {
        Some(self.id)
    }
}