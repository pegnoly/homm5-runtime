use std::collections::HashMap;

use homm5_types::{
    common::FileRef,
    creature::{AdvMapCreatureShared, CreatureVisual},
};
use quick_xml::{Reader, events::Event};

use crate::{
    core::Scan, error::ScanerError, pak::FileStructure, scaners::types_scaner::GameTypeItem,
    utils::configure_path,
};

pub struct CreatureScaner {
    pub id: i32,
    pub types_data: Vec<GameTypeItem>,
}

impl CreatureScaner {
    fn check_visual(
        &self,
        file_key: &String,
        content: &str,
        files: &HashMap<String, FileStructure>,
    ) -> Option<CreatureVisual> {
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(content);
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break None,
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"CreatureVisual" {
                        let end = e.to_end().into_owned();
                        let possible_text = reader.read_text(end.name());
                        match possible_text {
                            Ok(text) => {
                                let text = text.to_string();
                                let de_res: Result<CreatureVisual, quick_xml::DeError> =
                                    quick_xml::de::from_str(&format!(
                                        "<CreatureVisual>{text}</CreatureVisual>"
                                    ));
                                match de_res {
                                    Ok(visual) => {
                                        let name = configure_path(
                                            visual
                                                .CreatureNameFileRef
                                                .as_ref()
                                                .unwrap()
                                                .href
                                                .as_ref(),
                                            file_key,
                                            files,
                                        );
                                        let desc = configure_path(
                                            visual
                                                .DescriptionFileRef
                                                .as_ref()
                                                .unwrap()
                                                .href
                                                .as_ref(),
                                            file_key,
                                            files,
                                        );
                                        let icon_key = visual
                                            .Icon128
                                            .as_ref()
                                            .unwrap()
                                            .href
                                            .as_ref()
                                            .unwrap_or(&String::new())
                                            .replace("#xpointer(/Texture)", "");
                                        let icon = configure_path(Some(&icon_key), file_key, files);
                                        break Some(CreatureVisual {
                                            CreatureNameFileRef: Some(FileRef { href: Some(name) }),
                                            DescriptionFileRef: Some(FileRef { href: Some(desc) }),
                                            Icon128: Some(FileRef { href: Some(icon) }),
                                        });
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
                            Err(_e) => println!("error reading file content: {file_key}"),
                        }
                    }
                }
                _ => (),
            }
            buf.clear();
        }
    }
}

impl Scan for CreatureScaner {
    type Output = super::model::Model;
    fn scan(
        &mut self,
        file_key: &str,
        entity: &FileStructure,
        files: &HashMap<String, FileStructure>,
    ) -> Result<Option<Self::Output>, ScanerError> {
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(&entity.content);
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break Ok(None),
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"Creature" {
                        let end = e.to_end().into_owned();
                        let possible_text = reader.read_text(end.name());
                        match possible_text {
                            Ok(text) => {
                                let text = text.to_string();
                                let de_res: Result<AdvMapCreatureShared, quick_xml::DeError> =
                                    quick_xml::de::from_str(&format!(
                                        "<Creature>{text}</Creature>"
                                    ));
                                self.id += 1;
                                match de_res {
                                    Ok(mut creature) => {
                                        //println!("Creature scanned: {:?}", &creature);
                                        let visual = creature.Visual.as_ref();
                                        if let Some(actual_visual) = visual {
                                            let visual_key = actual_visual
                                                .href
                                                .as_ref()
                                                .unwrap()
                                                .replace("#xpointer(/CreatureVisual)", "")
                                                .trim_start_matches("/")
                                                .to_lowercase();
                                            //println!("visual key: {}", &visual_key);
                                            let actual_visual_key =
                                                configure_path(Some(&visual_key), file_key, files);
                                            let visual_file = files.get(&actual_visual_key);
                                            match visual_file {
                                                Some(actual_visual_file) => {
                                                    let visual_checked = self.check_visual(
                                                        &actual_visual_key,
                                                        &actual_visual_file.content,
                                                        files,
                                                    );
                                                    creature.VisualExplained = visual_checked;
                                                }
                                                None => println!(
                                                    "Can't find visual of {}",
                                                    &actual_visual_key
                                                ),
                                            }
                                        }
                                        let mut db_model = super::model::Model::from(creature);
                                        db_model.id = self.id;
                                        db_model.game_id = self
                                            .types_data
                                            .iter()
                                            .find(|item| item.value == self.id)
                                            .unwrap()
                                            .name
                                            .clone();
                                        if !db_model.name_txt.is_empty() {
                                            if let Some(name_data) = files.get(&db_model.name_txt) {
                                                db_model.name = name_data.content.clone();
                                            }
                                        }
                                        if !db_model.desc_txt.is_empty() {
                                            if let Some(desc_data) = files.get(&db_model.desc_txt) {
                                                db_model.desc = desc_data.content.clone();
                                            }
                                        }
                                        if self.id < 180 {
                                            db_model.xdb = Some(entity.content.to_string());
                                        }
                                        db_model.xdb_path = file_key.to_string();
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
                            Err(_e) => println!("error reading file content: {file_key}"),
                        }
                    }
                }
                _ => (),
            }
            buf.clear();
        }
    }
}
