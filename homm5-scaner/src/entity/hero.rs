use crate::{
    entity::{Scan, Output, configure_path, CollectFiles},
    pak::FileStructure
};
use quick_xml::{Reader, events::Event};
use std::collections::HashMap;
use homm5_types::{common::FileRef, hero::AdvMapHeroShared};

impl Output for AdvMapHeroShared {
    type ID = String;
    fn to_lua(&self, _id: Option<String>) -> String {
        let is_scenario_lua = if self.ScenarioHero == true {"1"} else {"nil"};
        format!(
            "\t[\"{}\"] = {{
        is_scenario = {},
        hero_class = {},
        spec = {},
        spec_name = \"{}\",
        spec_desc = \"{}\",
        spec_icon = \"{}\",
        icon = \"{}\",
        town = {},
        name = \"{}\",
        bio = \"{}\"
    }},\n", 
            self.InternalName, 
            is_scenario_lua, 
            self.Class, 
            self.Specialization, 
            self.SpecializationNameFileRef.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()), 
            self.SpecializationDescFileRef.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()), 
            self.SpecializationIcon.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()), 
            self.FaceTexture.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()),
            self.TownType,
            self.Editable.NameFileRef.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()),
            self.Editable.BiographyFileRef.as_ref().unwrap().href.as_ref().unwrap_or(&String::new())
        )
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

pub struct HeroFileCollector {}

impl CollectFiles for HeroFileCollector {
    fn collect(&self, files: &HashMap<String, FileStructure>, collected_files: &mut Vec<(String, FileStructure)>) {
        files.iter()
            .filter(|f| {
                f.1.content.contains("AdvMapHeroShared") && f.1.content.contains("ScenarioHero")
            })
            .for_each(|f| {
                collected_files.push((f.0.clone(), f.1.clone()))
            });
    }
}

pub struct HeroScaner {}

impl Scan<String> for HeroScaner {
    fn scan(&mut self, file_key: &String, entity: &String, files: &HashMap<String, FileStructure>) -> Option<Box<dyn Output<ID = String>>> {
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
                        b"AdvMapHeroShared" => {
                            let end = e.to_end().into_owned();
                            let possible_text = reader.read_text(end.name());
                            match possible_text {
                                Ok(text) => {
                                    let text = text.to_string();
                                    let de_res: Result<AdvMapHeroShared, quick_xml::DeError> = quick_xml::de::from_str(&format!("<AdvMapHeroShared>{}</AdvMapHeroShared>", text));
                                    match de_res {
                                        Ok(mut hero) => {
                                            let spec_name = configure_path(hero.SpecializationNameFileRef.as_ref().unwrap().href.as_ref(), file_key, files);
                                            let spec_desc = configure_path(hero.SpecializationDescFileRef.as_ref().unwrap().href.as_ref(), file_key, files);
                                            let spec_icon = configure_path(
                                                Some(&hero.SpecializationIcon.as_ref().unwrap().href.as_ref()
                                                    .unwrap_or(&String::new()).replace("#xpointer(/Texture)", "")), 
                                                    file_key, 
                                                    files
                                            );
                                            let icon = configure_path(
                                                Some(&hero.FaceTexture.as_ref().unwrap().href.as_ref()
                                                .unwrap_or(&String::new()).replace("#xpointer(/Texture)", "")), 
                                                file_key, 
                                                files
                                            );
                                            let name = configure_path(hero.Editable.NameFileRef.as_ref().unwrap().href.as_ref(), file_key, files);
                                            let bio = configure_path(hero.Editable.BiographyFileRef.as_ref().unwrap().href.as_ref(), file_key, files);
                                            hero.SpecializationNameFileRef = Some(FileRef { href: Some(spec_name) });
                                            hero.SpecializationDescFileRef = Some(FileRef { href: Some(spec_desc) });
                                            hero.SpecializationIcon = Some(FileRef { href: Some(spec_icon) });
                                            hero.FaceTexture = Some(FileRef { href: Some(icon) });
                                            hero.Editable.NameFileRef = Some(FileRef { href: Some(name) });
                                            hero.Editable.BiographyFileRef = Some(FileRef { href: Some(bio) });
                                            break Some(Box::new(hero));
                                        }
                                        Err(e) => {
                                            println!("error while deserializing {:?}", e.to_string());
                                        }
                                    }
                                },
                                Err(e) => println!("error reading file content: {}, {}", file_key, e.to_string())
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

    fn get_id(&self) -> Option<String> {
        None
    }

}