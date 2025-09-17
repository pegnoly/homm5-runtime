use std::collections::HashMap;

use quick_xml::{Reader, events::Event};

use crate::{core::CollectFiles, error::ScanerError, pak::FileStructure};

pub struct SkillFileCollector;

impl CollectFiles for SkillFileCollector {
    fn collect(
        &self,
        files: &HashMap<String, FileStructure>,
        collected_files: &mut Vec<(String, FileStructure)>,
    ) -> Result<(), ScanerError> {
        let skills_xdb = files
            .iter()
            .find(|f| f.0 == "GameMechanics/RefTables/Skills.xdb".to_lowercase().as_str())
            .unwrap();
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(skills_xdb.1.content.as_str());
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break Ok(()),
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"Item" {
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name()).unwrap().to_string();
                        let text = format!("<Skill>{text}</Skill>");
                        collected_files.push((
                            "GameMechanics/RefTables/Skills.xdb".to_lowercase(),
                            FileStructure {
                                pak: skills_xdb.1.pak.clone(),
                                modified: skills_xdb.1.modified,
                                content: text,
                            },
                        ));
                    }
                }
                _ => (),
            }
            buf.clear();
        }
    }
}
