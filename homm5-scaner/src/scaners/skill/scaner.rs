use super::model;
use crate::{core::Scan, error::ScanerError, pak::FileStructure, scaners::{skill::model::{Model, NamePaths, Names}, types_scaner::GameTypeItem}, utils::configure_path};
use homm5_types::skill::Skill;
use itertools::Itertools;
use std::collections::HashMap;

pub struct SkillScaner {
    pub id: i32,
    pub game_types: Vec<GameTypeItem>
}

impl Scan for SkillScaner {
    type Output = model::Model;

    fn scan(
        &mut self,
        file_key: &str,
        entity: &str,
        files: &HashMap<String, FileStructure>,
    ) -> Result<Option<Self::Output>, ScanerError> {
        let skill_de: Result<Skill, quick_xml::DeError> =
            quick_xml::de::from_str(entity);
        match skill_de {
            Ok(skill) => {
                let actual_name_paths = if let Some(paths) = skill.obj.NameFileRef.names {
                    paths.iter().map(|p| {
                        configure_path(p.href.as_ref(), file_key, files)
                    })
                    .collect_vec()
                } else {
                    vec![]
                };
                let actual_names = actual_name_paths.iter()
                    .filter_map(|n| {
                        files.get(n).map(|name| name.content.clone())
                    })
                    .collect_vec();
                self.id += 1;
                let model = Model {
                    id: self.id,
                    game_id: skill.ID,
                    name_paths: NamePaths { paths: actual_name_paths },
                    names: Names { names: actual_names },
                    hero_class: skill.obj.HeroClass,
                    basic_skill: skill.obj.BasicSkillID
                };
                Ok(Some(model))
            }
            Err(e) => {
                println!("error deserializing skill {e}");
                Ok(None)
            }
        }
    }
}
