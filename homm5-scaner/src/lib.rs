mod error;
mod core;
mod utils;
mod scaners;
mod pak;
mod service;
pub mod prelude;


        // let art_collector = ArtFileCollector {};
        // let art_scaner = ArtScaner { id: 0 };
        // let mut art_scan_processor = ScanProcessor::new(
        //     String::from("MCCS_ARTIFACTS_GENERATED_TABLE"),
        //     String::from("arts.lua"),
        //     Box::new(art_collector),
        //     Box::new(art_scaner),
        // );
        // //
        // let creature_scaner = CreatureScaner { id: 0 };
        // let creature_collector = CreatureFileCollector {};
        // let mut creature_scan_processor = ScanProcessor::new(
        //     String::from("MCCS_CREATURE_GENERATED_TABLE"),
        //     String::from("creatures.lua"),
        //     Box::new(creature_collector),
        //     Box::new(creature_scaner),
        // );
        // //
        // let hero_scaner = HeroScaner::new();
        // let hero_collector = HeroFileCollector {};
        // let mut hero_scan_processor = ScanProcessor::new(
        //     String::from("MCCS_GENERATED_HEROES_TABLE"),
        //     String::from("heroes.lua"),
        //     Box::new(hero_collector),
        //     Box::new(hero_scaner),
        // );
        // //
        // let spell_collector = SpellFileCollector {};
        // let spell_scaner = SpellScaner { id: 0 };
        // let mut spell_scan_processor = ScanProcessor::new(
        //     String::from("MCCS_SPELL_GENERATED_TABLE"),
        //     String::from("spells.lua"),
        //     Box::new(spell_collector),
        //     Box::new(spell_scaner),
        // );

        // let creatures_generated_file = creature_scan_processor.run(&files);
        // let heroes_generated_file = hero_scan_processor.run(&files);
        // let artifacts_generated_file = art_scan_processor.run(&files);
        // let spells_generated_file = spell_scan_processor.run(&files);
        // let zip_file = std::fs::File::create(&self.output_path).unwrap();
        // let mut map_zipped = zip::ZipWriter::new(zip_file);
        // map_zipped
        //     .start_file("scripts/generated/creatures.lua", FileOptions::default())
        //     .unwrap();
        // map_zipped
        //     .write_all(creatures_generated_file.0.as_bytes())
        //     .unwrap();
        // map_zipped
        //     .start_file("scripts/generated/heroes.lua", FileOptions::default())
        //     .unwrap();
        // map_zipped
        //     .write_all(heroes_generated_file.0.as_bytes())
        //     .unwrap();
        // map_zipped
        //     .start_file("scripts/generated/artifacts.lua", FileOptions::default())
        //     .unwrap();
        // map_zipped
        //     .write_all(artifacts_generated_file.0.as_bytes())
        //     .unwrap();
        // map_zipped
        //     .start_file("scripts/generated/spells.lua", FileOptions::default())
        //     .unwrap();
        // map_zipped
        //     .write_all(spells_generated_file.0.as_bytes())
        //     .unwrap();
        // map_zipped.finish().unwrap();

        // let mut file = std::fs::File::create("C:\\H5ToE\\data\\heroes.json").unwrap();
        // file.write_all(&mut heroes_generated_file.1.as_bytes()).unwrap();