use homm5_types::hero_class::HeroClass;

use super::model;
use crate::{core::Scan, error::ScanerError, pak::FileStructure, utils::configure_path};
use std::collections::HashMap;

pub struct HeroClassScaner {
    pub id: i32,
}

impl Scan for HeroClassScaner {
    type Output = model::Model;

    fn scan(
        &mut self,
        file_key: &str,
        entity: &str,
        files: &HashMap<String, FileStructure>,
    ) -> Result<Option<Self::Output>, ScanerError> {
        let class_de: Result<HeroClass, quick_xml::DeError> = quick_xml::de::from_str(entity);
        match class_de {
            Ok(class) => {
                let name = configure_path(
                    class.obj.NameFileRef.as_ref().unwrap().href.as_ref(),
                    file_key,
                    files,
                );
                self.id += 1;
                let mut db_model = model::Model::from(class);
                db_model.id = self.id;
                if let Some(data) = files.get(&name) {
                    db_model.name = data.content.clone();
                }
                Ok(Some(db_model))
            }
            Err(e) => {
                println!("error deserializing hero class {e}");
                Ok(None)
            }
        }
    }
}
