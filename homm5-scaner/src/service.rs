use std::{collections::HashMap, path::PathBuf};

use sea_orm::{sqlx::SqlitePool, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, SqlxSqlitePoolConnection};

use crate::{core::ScanProcessor, error::ScanerError, pak::{self, FileStructure, EXTENSIONS}, prelude::{ArtifactDBColumn, ArtifactDBModel, CreatureDBColumn, CreatureDBModel}, scaners::{self, prelude::{ArtFileCollector, ArtScaner, ArtifactDataOutput, CreatureDataOutput, CreatureFilesCollector, CreatureScaner, HeroDataOutput, HeroFilesCollector, HeroScaner, SpellDataOutput, SpellFileCollector, SpellScaner}}};

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

        let mut artifact_scan_processor = ScanProcessor::new(
            ArtFileCollector,
            ArtScaner { id: -1 }, 
            ArtifactDataOutput::new(&self.db)
        );

        let mut spell_scan_processor = ScanProcessor::new(
            SpellFileCollector,
            SpellScaner { id: 0 }, 
            SpellDataOutput::new(&self.db)
        );

        creature_scan_processor.run(&mut files).await.unwrap();
        hero_scan_processor.run(&mut files).await.unwrap();
        artifact_scan_processor.run(&mut files).await.unwrap();
        spell_scan_processor.run(&mut files).await.unwrap();
    }

    pub async fn get_artifact_models(&self) -> Result<Vec<ArtifactDBModel>, ScanerError> {
        Ok(scaners::prelude::ArtifactDBEntity::find().filter(ArtifactDBColumn::IsGeneratable.eq(true)).all(&self.db).await?)
    }

    pub async fn get_creature_models(&self) -> Result<Vec<CreatureDBModel>, ScanerError> {
        Ok(
            scaners::prelude::CreatureDBEntity::find()
                .filter(CreatureDBColumn::Initiative.between(2, 30))
                .filter(CreatureDBColumn::Speed.between(2, 30))
                .filter(CreatureDBColumn::Power.between(2, 100000))
                .all(&self.db)
                .await?
        )
    }
}