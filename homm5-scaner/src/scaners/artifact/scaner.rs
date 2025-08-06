use super::model;
use crate::{core::Scan, error::ScanerError, pak::FileStructure, utils::configure_path};
use homm5_types::{art::AdvMapArtifactShared, common::FileRef};
use std::collections::HashMap;

pub struct ArtScaner {
    pub id: i32,
}

impl Scan for ArtScaner {
    type Output = model::Model;

    fn scan(
        &mut self,
        file_key: &str,
        entity: &str,
        files: &HashMap<String, FileStructure>,
    ) -> Result<Option<Self::Output>, ScanerError> {
        let art_de: Result<AdvMapArtifactShared, quick_xml::DeError> =
            quick_xml::de::from_str(entity);
        match art_de {
            Ok(mut art) => {
                let name = configure_path(
                    art.NameFileRef.as_ref().unwrap().href.as_ref(),
                    file_key,
                    files,
                );
                let desc = configure_path(
                    art.DescriptionFileRef.as_ref().unwrap().href.as_ref(),
                    file_key,
                    files,
                );
                let icon_key = art
                    .Icon
                    .as_ref()
                    .unwrap()
                    .href
                    .as_ref()
                    .unwrap_or(&String::new())
                    .replace("#xpointer(/Texture)", "")
                    .to_lowercase();
                let icon = configure_path(Some(&icon_key), file_key, files);
                art.NameFileRef = Some(FileRef { href: Some(name) });
                art.DescriptionFileRef = Some(FileRef { href: Some(desc) });
                art.Icon = Some(FileRef { href: Some(icon) });
                self.id += 1;
                let mut db_model = model::Model::from(art);
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
                Ok(Some(db_model))
            }
            Err(e) => {
                println!("error deserializing artifact {e}");
                Ok(None)
            }
        }
    }
}
