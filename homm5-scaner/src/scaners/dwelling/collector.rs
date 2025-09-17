use std::{collections::HashMap, sync::LazyLock};

use crate::{
    core::CollectFiles, error::ScanerError, pak::FileStructure,
    scaners::dwelling::model::DwellingType,
};

static DWELLS_DATA: LazyLock<HashMap<DwellingType, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        (
            DwellingType::HumanT1Dwelling,
            "mapobjects/haven/peasanthut.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::HumanT2Dwelling,
            "mapobjects/haven/archerstower.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::HumanT3Dwelling,
            "mapobjects/haven/barracks.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::HumansDwelling,
            "mapobjects/haven/heaven_military_post.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::InfernoT1Dwelling,
            "mapobjects/inferno/impcrucible.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::InfernoT2Dwelling,
            "mapobjects/inferno/demongate.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::InfernoT3Dwelling,
            "mapobjects/inferno/kennels.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::InfernoDwelling,
            "mapobjects/inferno/infernomilitarypost.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::NecropolisT1Dwelling,
            "mapobjects/necropolis/graveyard.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::NecropolisT2Dwelling,
            "mapobjects/necropolis/forgotten_mound.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::NecropolisT3Dwelling,
            "mapobjects/necropolis/ruined_tower.xdb",
        ),
        (
            DwellingType::NecropolisDwelling,
            "mapobjects/necropolis/necropolis_military_post.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::ElvesT1Dwelling,
            "mapobjects/preserve/fairie_tree.xdb",
        ),
        (
            DwellingType::ElvesT2Dwelling,
            "mapobjects/preserve/wood_guard_quarters.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::ElvesT3Dwelling,
            "mapobjects/preserve/high_cabins.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::ElvesDwelling,
            "mapobjects/preserve/preserve_military_post.xdb",
        ),
        (
            DwellingType::LigaT1Dwelling,
            "mapobjects/dungeon/battle_academy.xdb",
        ),
        (
            DwellingType::LigaT2Dwelling,
            "mapobjects/dungeon/blood_stone.xdb",
        ),
        (DwellingType::LigaT3Dwelling, "mapobjects/dungeon/maze.xdb"),
        (
            DwellingType::LigaDwelling,
            "mapobjects/dungeon/dungeon_military_post.xdb",
        ),
        (
            DwellingType::MagesT1Dwelling,
            "mapobjects/academy/workshop.xdb",
        ),
        (
            DwellingType::MagesT2Dwelling,
            "mapobjects/academy/stone_parapet.xdb",
        ),
        (
            DwellingType::MagesT3Dwelling,
            "mapobjects/academy/golemforge.xdb",
        ),
        (
            DwellingType::MagesDwelling,
            "mapobjects/academy/academy_military_post.xdb",
        ),
        (
            DwellingType::DwarfsT1Dwelling,
            "mapobjects/dwarven/dwarvendwelling.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::DwarfsT2Dwelling,
            "mapobjects/dwarven/dwarvendwelling02.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::DwarfsT3Dwelling,
            "mapobjects/dwarven/dwarvendwelling03.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::DwarfsDwelling,
            "mapobjects/dwarven/dwarvendwelling04.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::HordeT1Dwelling,
            "mapobjects/orcs/orcishdwelling01.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::HordeT2Dwelling,
            "mapobjects/orcs/orcishdwelling02.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::HordeT3Dwelling,
            "mapobjects/orcs/orcishdwelling03.(advmapdwellingshared).xdb",
        ),
        (
            DwellingType::HordeDwelling,
            "mapobjects/orcs/orcishdwelling04.(advmapdwellingshared).xdb",
        ),
    ])
});

pub struct DwellingsFileCollector;

impl CollectFiles for DwellingsFileCollector {
    fn collect(
        &self,
        files: &HashMap<String, FileStructure>,
        collected_files: &mut Vec<(String, FileStructure)>,
    ) -> Result<(), ScanerError> {
        DWELLS_DATA.iter().for_each(|(dwell_type, path)| {
            if let Some(data) = files.get(*path) {
                collected_files.push((dwell_type.to_string(), data.clone()));
            }
        });
        Ok(())
    }
}
