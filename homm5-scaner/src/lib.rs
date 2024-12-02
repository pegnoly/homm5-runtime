use std::{collections::HashMap, io::Write, path::PathBuf};

use entity::{art::{ArtFileCollector, ArtScaner}, 
creature::{CreatureFileCollector, CreatureScaner}, hero::{HeroFileCollector, HeroScaner}, 
spell::{SpellFileCollector, SpellScaner}, ScanProcessor};

use pak::FileStructure;
use zip::write::FileOptions;

pub mod pak;
pub mod entity;
pub mod output;

pub struct ScanExecutor {
    data_path: PathBuf
}

impl ScanExecutor {
    pub fn new(path: PathBuf) -> Self {
        ScanExecutor {
            data_path: path
        }
    }

    pub async fn run(&self) {
        let mut files: HashMap<String, FileStructure> = HashMap::new();
        let entries = std::fs::read_dir(&self.data_path).unwrap();
        let mut paks: Vec<String> = vec![];
        for f in entries {
            let name = f.as_ref().unwrap().file_name();
            paks.push(name.to_str().unwrap().to_string());
            if name.to_str().unwrap().ends_with(".pak") {
                pak::check_pak(self.data_path.join(&name), &mut files);
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
            self.data_path.join("MCCS_GeneratedFiles.pak")
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