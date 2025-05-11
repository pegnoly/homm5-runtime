use std::collections::HashMap;

use homm5_types::{common::FileRef, hero::AdvMapHeroShared};
use quick_xml::{events::Event, Reader};

use crate::{core::Scan, error::ScanerError, pak::FileStructure, utils::configure_path};

use super::model;

pub struct HeroScaner {
    id: String
}

impl HeroScaner {
    pub fn new() -> Self {
        HeroScaner { 
            id: String::new()
        }
    }
}

impl Scan for HeroScaner {
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
                    b"AdvMapHeroShared" => {
                        let end = e.to_end().into_owned();
                        let possible_text = reader.read_text(end.name());
                        match possible_text {
                            Ok(text) => {
                                let text = text.to_string();
                                let de_res: Result<AdvMapHeroShared, quick_xml::DeError> =
                                    quick_xml::de::from_str(&format!(
                                        "<AdvMapHeroShared>{}</AdvMapHeroShared>",
                                        text
                                    ));
                                match de_res {
                                    Ok(mut hero) => {
                                        self.id = file_key.clone();
                                        let spec_name = configure_path(
                                            hero.SpecializationNameFileRef
                                                .as_ref()
                                                .unwrap()
                                                .href
                                                .as_ref(),
                                            file_key,
                                            files,
                                        );
                                        let spec_desc = configure_path(
                                            hero.SpecializationDescFileRef
                                                .as_ref()
                                                .unwrap()
                                                .href
                                                .as_ref(),
                                            file_key,
                                            files,
                                        );
                                        let spec_icon = configure_path(
                                            Some(
                                                &hero
                                                    .SpecializationIcon
                                                    .as_ref()
                                                    .unwrap()
                                                    .href
                                                    .as_ref()
                                                    .unwrap_or(&String::new())
                                                    .replace("#xpointer(/Texture)", ""),
                                            ),
                                            file_key,
                                            files,
                                        );
                                        let icon = configure_path(
                                            Some(
                                                &hero
                                                    .FaceTexture
                                                    .as_ref()
                                                    .unwrap()
                                                    .href
                                                    .as_ref()
                                                    .unwrap_or(&String::new())
                                                    .replace("#xpointer(/Texture)", ""),
                                            ),
                                            file_key,
                                            files,
                                        );
                                        let name = configure_path(
                                            hero.Editable
                                                .NameFileRef
                                                .as_ref()
                                                .unwrap()
                                                .href
                                                .as_ref(),
                                            file_key,
                                            files,
                                        );
                                        let bio = configure_path(
                                            hero.Editable
                                                .BiographyFileRef
                                                .as_ref()
                                                .unwrap()
                                                .href
                                                .as_ref(),
                                            file_key,
                                            files,
                                        );
                                        hero.SpecializationNameFileRef = Some(FileRef {
                                            href: Some(spec_name),
                                        });
                                        hero.SpecializationDescFileRef = Some(FileRef {
                                            href: Some(spec_desc),
                                        });
                                        hero.SpecializationIcon = Some(FileRef {
                                            href: Some(spec_icon),
                                        });
                                        hero.FaceTexture = Some(FileRef { href: Some(icon) });
                                        hero.Editable.NameFileRef =
                                            Some(FileRef { href: Some(name) });
                                        hero.Editable.BiographyFileRef =
                                            Some(FileRef { href: Some(bio) });

                                        let mut db_model = model::Model::from(hero);
                                        db_model.id = self.id.clone();
                                        if !db_model.spec_name_txt.is_empty() {
                                            if let Some(data) = files.get(&db_model.spec_name_txt) {
                                                db_model.spec_name = data.content.clone();
                                            }
                                        }
                                        if !db_model.spec_name_txt.is_empty() {
                                            if let Some(data) = files.get(&db_model.spec_desc_txt) {
                                                db_model.spec_desc = data.content.clone();
                                            }
                                        }
                                        if !db_model.editable.name_txt.is_empty() {
                                            if let Some(data) = files.get(&db_model.editable.name_txt) {
                                                db_model.editable.name = data.content.clone();
                                            }
                                        }
                                        if !db_model.editable.bio_txt.is_empty() {
                                            if let Some(data) = files.get(&db_model.editable.bio_txt) {
                                                db_model.editable.bio = data.content.clone();
                                            }
                                        }                     
                                        break Ok(Some(db_model))                   
                                    }
                                    Err(e) => {
                                        println!(
                                            "error while deserializing hero file {}: {:?}",
                                            file_key,
                                            e.to_string()
                                        );
                                    }
                                }
                            }
                            Err(e) => println!(
                                "error reading file content: {}, {}",
                                file_key,
                                e.to_string()
                            ),
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