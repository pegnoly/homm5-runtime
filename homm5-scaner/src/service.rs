use std::{collections::HashMap, path::PathBuf};

use sea_orm::{sqlx::SqlitePool, DatabaseConnection, SqlxSqlitePoolConnection};

use crate::{core::ScanProcessor, pak::{self, FileStructure, EXTENSIONS}, scaners::prelude::{CreatureDataOutput, CreatureFilesCollector, CreatureScaner, HeroDataOutput, HeroFilesCollector, HeroScaner}};

pub struct ScanerService {
    db: DatabaseConnection
}

impl ScanerService {
    pub fn new(pool: SqlitePool) -> Self {
        ScanerService {
            db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool))
        }
    }

    pub async fn run(&self, paths_to_scan: Vec<PathBuf>, output_path: PathBuf) {
        let mut files: HashMap<String, FileStructure> = HashMap::new();
        for dir in paths_to_scan {
            let entries = std::fs::read_dir(&dir).unwrap();
            for f in entries {
                let name = f.as_ref().unwrap().file_name();
                if EXTENSIONS
                    .iter()
                    .any(|e| name.to_str().unwrap().ends_with(e))
                {
                    pak::check_pak(dir.join(&name), &mut files);
                }
            }
        }

        let mut creature_scan_processor = ScanProcessor::new(
            CreatureFilesCollector, 
            CreatureScaner {id: -1}, 
            CreatureDataOutput::new(&self.db)
        );

        let mut hero_scan_processor = ScanProcessor::new(
            HeroFilesCollector, 
            HeroScaner::new(), 
            HeroDataOutput::new(&self.db)
        );

        creature_scan_processor.run(&mut files).await.unwrap();
        hero_scan_processor.run(&mut files).await.unwrap();
    }
}