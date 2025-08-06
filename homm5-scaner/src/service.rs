use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
    SqlxSqlitePoolConnection, prelude::Expr, sea_query::SimpleExpr, sqlx::SqlitePool,
};
use std::{collections::HashMap, path::PathBuf};

use crate::{
    core::ScanProcessor, error::ScanerError, pak::{self, FileStructure, EXTENSIONS}, prelude::{
        AbilityDBColumn, AbilityDBEntity, AbilityDBModel, AbilityDataOutput, AbilityFileCollector, AbilityScaner, ArtifactDBColumn, ArtifactDBEntity, ArtifactDBModel, CreatureDBColumn, CreatureDBEntity, CreatureDBModel, DwellingDataOutput, DwellingScaner, DwellingsFileCollector, HeroClassDataOutput, HeroClassFileCollector, HeroClassScaner, HeroDBColumn, HeroDBEntity, HeroDBModel, MagicSchoolType, SkillDBColumn, SkillDBEntity, SkillDBModel, SkillDataOutput, SkillFileCollector, SkillScaner, SpellDBColumn, SpellDBEntity, SpellDBModel, Town, TypesXmlScaner, BASE_SKILLS
    }, scaners::{
        self,
        prelude::{
            ArtFileCollector, ArtScaner, ArtifactDataOutput, CreatureDataOutput,
            CreatureFilesCollector, CreatureScaner, HeroDataOutput, HeroFilesCollector, HeroScaner,
            SpellDataOutput, SpellFileCollector, SpellScaner,
        },
    }
};

pub struct ScanerService {
    db: DatabaseConnection,
}

impl ScanerService {
    pub fn new(pool: SqlitePool) -> Self {
        ScanerService {
            db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)),
        }
    }

    pub async fn run(&self, paths_to_scan: Vec<PathBuf>, output_path: PathBuf) -> Result<(), ScanerError> {
        let mut files: HashMap<String, FileStructure> = HashMap::new();
        for dir in paths_to_scan {
            let entries = std::fs::read_dir(&dir)?;
            for f in entries {
                let name = f?.file_name();
                if EXTENSIONS
                    .iter()
                    .any(|e| name.to_str().unwrap_or("").ends_with(e))
                {
                    pak::check_pak(dir.join(&name), &mut files);
                }
            }
        }

        let mut types_xml_scaner = TypesXmlScaner { creature_items: vec![], skills_items: vec![], spells_items: vec![] };
        types_xml_scaner.parse(&files)?;

        let mut hero_class_scan_processor = ScanProcessor::new(
            HeroClassFileCollector, 
            HeroClassScaner { id: -1 }, 
            HeroClassDataOutput::new(&self.db)
        );

        let mut creature_scan_processor = ScanProcessor::new(
            CreatureFilesCollector,
            CreatureScaner { id: -1, types_data: types_xml_scaner.creature_items },
            CreatureDataOutput::new(&self.db),
        );

        let mut hero_scan_processor = ScanProcessor::new(
            HeroFilesCollector,
            HeroScaner::new(),
            HeroDataOutput::new(&self.db),
        );

        let mut artifact_scan_processor = ScanProcessor::new(
            ArtFileCollector,
            ArtScaner { id: -1 },
            ArtifactDataOutput::new(&self.db),
        );

        let mut spell_scan_processor = ScanProcessor::new(
            SpellFileCollector,
            SpellScaner { id: 0, game_types: types_xml_scaner.spells_items },
            SpellDataOutput::new(&self.db),
        );

        let mut ability_scan_processor = ScanProcessor::new(
            AbilityFileCollector, 
            AbilityScaner { id: -1 }, 
            AbilityDataOutput::new(&self.db)
        );

        let mut skill_scan_processor = ScanProcessor::new(
            SkillFileCollector, 
            SkillScaner { id: -1, game_types: types_xml_scaner.skills_items }, 
            SkillDataOutput::new(&self.db)
        );

        let mut dwelling_scan_processor = ScanProcessor::new(
            DwellingsFileCollector, 
            DwellingScaner, 
            DwellingDataOutput::new()
        );

        let zip_file = std::fs::File::create(output_path)?;
        let mut zip_writer = zip::ZipWriter::new(zip_file);

        hero_class_scan_processor
            .run(&files, &mut zip_writer)
            .await?;
        creature_scan_processor
            .run(&files, &mut zip_writer)
            .await?;
        hero_scan_processor
            .run(&files, &mut zip_writer)
            .await?;
        artifact_scan_processor
            .run(&files, &mut zip_writer)
            .await?;
        spell_scan_processor
            .run(&files, &mut zip_writer)
            .await?;
        ability_scan_processor
            .run(&files, &mut zip_writer)
            .await?;
        skill_scan_processor
            .run(&files, &mut zip_writer)
            .await?;
        dwelling_scan_processor
            .run(&files, &mut zip_writer)
            .await?;

        Ok(())
    }

    pub async fn get_heroes_models(&self, town: Town) -> Result<Vec<HeroDBModel>, ScanerError> {
        let heroes = HeroDBEntity::find().filter(HeroDBColumn::Town.eq(town)).all(&self.db).await?;
        Ok(heroes)
    }

    pub async fn get_artifact_models(&self) -> Result<Vec<ArtifactDBModel>, ScanerError> {
        Ok(scaners::prelude::ArtifactDBEntity::find()
            .filter(ArtifactDBColumn::IsGeneratable.eq(true))
            .all(&self.db)
            .await?)
    }

    pub async fn get_creature_models(&self) -> Result<Vec<CreatureDBModel>, ScanerError> {
        Ok(scaners::prelude::CreatureDBEntity::find()
            .filter(CreatureDBColumn::Initiative.between(2, 30))
            .filter(CreatureDBColumn::Speed.between(2, 30))
            .filter(CreatureDBColumn::Power.between(2, 100000))
            .all(&self.db)
            .await?)
    }

    pub async fn get_all_creature_models(&self) -> Result<Vec<CreatureDBModel>, ScanerError> {
        Ok(CreatureDBEntity::find().all(&self.db).await?)
    }

    pub async fn get_average_counts_for_power(
        &self,
        power: i32,
        towns: Vec<Town>,
        tiers: Vec<i32>,
    ) -> Result<Option<i32>, ScanerError> {
        let towns_condition = Condition::all().add_option(if towns.len() > 0 {
            Some(Expr::col(CreatureDBColumn::Town).is_in(towns))
        } else {
            None::<SimpleExpr>
        });
        let tiers_condition = Condition::all().add_option(if tiers.len() > 0 {
            Some(Expr::col(CreatureDBColumn::Tier).is_in(tiers))
        } else {
            None::<SimpleExpr>
        });

        let data = CreatureDBEntity::find()
            .filter(towns_condition)
            .filter(tiers_condition)
            .filter(CreatureDBColumn::Id.between(1, 179))
            .select_only()
            .column_as(CreatureDBColumn::Power.sum(), "sum")
            .column_as(CreatureDBColumn::Power.count(), "count")
            .into_tuple::<(i32, i64)>()
            .one(&self.db)
            .await?;

        if let Some(data) = data {
            let average_power = (data.0 as f64 / data.1 as f64).ceil();
            Ok(Some((power as f64 / average_power).ceil() as i32))
        } else {
            Ok(None)
        }
    }

    pub async fn get_average_concrete_creatures_count_for_power(
        &self,
        power: i32,
        creatures: Vec<i32>
    ) -> Result<Option<i32>, ScanerError> {
        let condition = Condition::all()
            .add_option(if creatures.len() > 0 {
                Some(Expr::col(CreatureDBColumn::Id).is_in(creatures))
            } else {
                None::<SimpleExpr>
            });
        let data = CreatureDBEntity::find()
            .filter(condition)
            .select_only()
            .column_as(CreatureDBColumn::Power.sum(), "sum")
            .column_as(CreatureDBColumn::Power.count(), "count")
            .into_tuple::<(i32, i64)>()
            .one(&self.db)
            .await?;

        if let Some(data) = data {
            let average_power = (data.0 as f64 / data.1 as f64).ceil();
            Ok(Some((power as f64 / average_power).ceil() as i32))
        } else {
            Ok(None)
        }
    }

    pub async fn get_average_artifacts_cost(
        &self,
        artifacts: Vec<i32>
    ) -> Result<Option<i32>, ScanerError> {
        let condition = Condition::all()
            .add_option(if artifacts.len() > 0 {
                Some(Expr::col(ArtifactDBColumn::Id).is_in(artifacts))
            } else {
                None::<SimpleExpr>
            });
        let data = ArtifactDBEntity::find()
            .filter(condition)
            .select_only()
            .column_as(ArtifactDBColumn::Cost.sum(), "sum")
            .column_as(ArtifactDBColumn::Cost.count(), "count")
            .into_tuple::<(i32, i64)>()
            .one(&self.db)
            .await?;
        
        if let Some(data) = data {
            let average_cost = (data.0 as f64 / data.1 as f64).ceil();
            Ok(Some(average_cost as i32))
        } else {
            Ok(None)
        }
    }

    pub async fn get_abilities(
        &self
    ) -> Result<Vec<AbilityDBModel>, ScanerError> {
        Ok(AbilityDBEntity::find().filter(AbilityDBColumn::Id.between(1, 200)).all(&self.db).await?)
    }

    pub async fn get_creature(
        &self,
        id: i32
    ) -> Result<Option<CreatureDBModel>, ScanerError> {
        Ok(CreatureDBEntity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn get_perks_for_skill(
        &self,
        skill: String
    ) -> Result<Vec<SkillDBModel>, ScanerError> {
        Ok(SkillDBEntity::find().filter(SkillDBColumn::BasicSkill.eq(skill)).all(&self.db).await?)
    }

    pub async fn get_base_skills(&self) -> Result<Vec<SkillDBModel>, ScanerError> {
        Ok(
            SkillDBEntity::find()
                .filter(SkillDBColumn::GameId.is_in(BASE_SKILLS.clone()))
                .all(&self.db)
                .await?
        )
    }

    pub async fn get_spells_for_school(&self, school: MagicSchoolType) -> Result<Vec<SpellDBModel>, ScanerError> {
        Ok(SpellDBEntity::find().filter(SpellDBColumn::School.eq(school)).all(&self.db).await?)
    }
}