use std::collections::HashMap;

use crate::{core::CollectFiles, error::ScanerError, pak::FileStructure};

pub struct HeroFilesCollector;

impl CollectFiles for HeroFilesCollector {
    fn collect(
        &self,
        files: &HashMap<String, FileStructure>,
        collected_files: &mut Vec<(String, FileStructure)>,
    ) -> Result<(), ScanerError> {
        files
            .iter()
            .filter(|f| {
                f.1.content.contains("AdvMapHeroShared") && f.1.content.contains("ScenarioHero")
            })
            .for_each(|f| collected_files.push((f.0.clone(), f.1.clone())));
        Ok(())
    }
}
