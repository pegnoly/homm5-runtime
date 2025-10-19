use homm5_types::ability::AbilityShared;

use super::model;
use crate::{core::Scan, error::ScanerError, pak::FileStructure, utils::configure_path};
use std::collections::HashMap;

pub struct AbilityScaner {
    pub id: i32,
}

impl Scan for AbilityScaner {
    type Output = model::Model;

    fn scan(
        &mut self,
        file_key: &str,
        entity: &FileStructure,
        files: &HashMap<String, FileStructure>,
    ) -> Result<Option<Self::Output>, ScanerError> {
        let ability_de: Result<AbilityShared, quick_xml::DeError> = quick_xml::de::from_str(&entity.content);
        match ability_de {
            Ok(ability) => {
                let name = configure_path(
                    ability.obj.NameFileRef.as_ref().unwrap().href.as_ref(),
                    file_key,
                    files,
                );
                // let desc = configure_path(
                //     ability.obj.DescriptionFileRef.as_ref().unwrap().href.as_ref(),
                //     file_key,
                //     files,
                // );
                self.id += 1;
                let mut db_model = model::Model::from(ability);
                db_model.id = self.id;
                if let Some(data) = files.get(&name) {
                    db_model.name = data.content.clone();
                }
                Ok(Some(db_model))
            }
            Err(e) => {
                println!("error deserializing ability {e}");
                Ok(None)
            }
        }
    }
}
