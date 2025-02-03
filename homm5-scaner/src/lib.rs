use std::{collections::HashMap, io::Write, path::PathBuf};

use entity::{art::{ArtFileCollector, ArtScaner}, 
creature::{CreatureFileCollector, CreatureScaner}, hero::{HeroFileCollector, HeroScaner}, 
spell::{SpellFileCollector, SpellScaner}, ScanProcessor};

use pak::FileStructure;
use zip::write::FileOptions;

pub mod pak;
pub mod entity;
pub mod output;

const EXTENSIONS: [&str; 4] = [".pak", ".h5m", ".h5c", ".h5u"];

pub struct ScanExecutor {
    output_path: PathBuf,
    paths_to_scan: Vec<PathBuf>
}

impl ScanExecutor {
    pub fn new(output: PathBuf, paths: Vec<PathBuf>) -> Self {
        ScanExecutor {
            output_path: output,
            paths_to_scan: paths
        }
    }

    pub async fn run(&self) {
        let mut files: HashMap<String, FileStructure> = HashMap::new();
        for dir in &self.paths_to_scan {
            let entries = std::fs::read_dir(&dir).unwrap();
            for f in entries {
                let name = f.as_ref().unwrap().file_name();
                if EXTENSIONS.iter().any(|e| name.to_str().unwrap().ends_with(e)) {
                    pak::check_pak(dir.join(&name), &mut files);
                }
            }
        }
        let art_collector = ArtFileCollector{};
        let art_scaner = ArtScaner{id: 0};
        let mut art_scan_processor = ScanProcessor::new(
            String::from("MCCS_ARTIFACTS_GENERATED_TABLE"), 
           String::from("arts.lua"), 
        Box::new(art_collector),
        Box::new(art_scaner)
        );
        //
        let creature_scaner = CreatureScaner {id: 0};
        let creature_collector = CreatureFileCollector {};
        let mut creature_scan_processor = ScanProcessor::new(
            String::from("MCCS_CREATURE_GENERATED_TABLE"), 
           String::from("creatures.lua"), 
        Box::new(creature_collector),
        Box::new(creature_scaner)
        );
        //
        let hero_scaner = HeroScaner{};
        let hero_collector = HeroFileCollector{};
        let mut hero_scan_processor = ScanProcessor::new(
            String::from("MCCS_GENERATED_HEROES_TABLE"), 
           String::from("heroes.lua"), 
        Box::new(hero_collector),
        Box::new(hero_scaner)
        );
        //
        let spell_collector = SpellFileCollector{};
        let spell_scaner = SpellScaner{id: 0};
        let mut spell_scan_processor = ScanProcessor::new(
            String::from("MCCS_SPELL_GENERATED_TABLE"), 
           String::from("spells.lua"), 
        Box::new(spell_collector),
        Box::new(spell_scaner)
        );
    
        let creatures_generated_file = creature_scan_processor.run(&files);
        let heroes_generated_file = hero_scan_processor.run(&files);
        let artifacts_generated_file = art_scan_processor.run(&files);
        let spells_generated_file = spell_scan_processor.run(&files);
        let zip_file = std::fs::File::create(
            &self.output_path
        ).unwrap();
        let mut map_zipped = zip::ZipWriter::new(zip_file);
        map_zipped.start_file("scripts/generated/creatures.lua", FileOptions::default()).unwrap();
        map_zipped.write_all(creatures_generated_file.0.as_bytes()).unwrap();
        map_zipped.start_file("scripts/generated/heroes.lua", FileOptions::default()).unwrap();
        map_zipped.write_all(heroes_generated_file.0.as_bytes()).unwrap();
        map_zipped.start_file("scripts/generated/artifacts.lua", FileOptions::default()).unwrap();
        map_zipped.write_all(artifacts_generated_file.0.as_bytes()).unwrap();
        map_zipped.start_file("scripts/generated/spells.lua", FileOptions::default()).unwrap();
        map_zipped.write_all(spells_generated_file.0.as_bytes()).unwrap();
        map_zipped.finish().unwrap();
    }
}