use core::str;
use std::{collections::HashMap, io::Write};

use artifacts::ArtifactsModifier;
use buildings::BuildingsModifier;
use homm5_types::{art::AdvMapArtifact, building::AdvMapBuilding, creature::AdvMapMonster, hero::AdvMapHero};
pub use homm5_types::{common::FileRef, quest::{Objectives, Quest, QuestList}, Homm5Type};
use monsters::MonstersModifier;
use quick_xml::{events::{BytesDecl, BytesEnd, BytesStart, Event}, Reader, Writer};
use serde::{Deserialize, Serialize};

pub mod quest;
pub mod reserve_heroes;
pub mod artifacts;
pub mod buildings;
pub mod monsters;

#[derive(Serialize, Deserialize, Debug)]
pub struct  PlayerSpecific {
    #[serde(rename = "Item")]
    pub items: Option<Vec<QuestList>> 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Primary {
    #[serde(rename = "Common")]
    pub common: Option<QuestList>,
    #[serde(rename = "PlayerSpecific")]
    pub player_specific: PlayerSpecific
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Secondary {
    #[serde(rename = "Common")]
    pub common: Option<QuestList>,
    #[serde(rename = "PlayerSpecific")]
    pub player_specific: PlayerSpecific
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "Objectives")]
pub struct ObjectivesInfo {
    #[serde(rename = "Primary")]
    pub primary: Primary,
    #[serde(rename = "Secondary")]
    pub secondary: Secondary
}

pub trait GenerateBoilerplate {
    type Output: Homm5Type;
    type Additional;

    fn generate(&self, additional_data: Option<&Self::Additional>) -> Result<Self::Output, std::io::Error>;
}
pub struct ModifiersQueue {
    pub primary_quests: Vec<Quest>,
    pub secondary_quests: Vec<Quest>,
    buildings_modifier: BuildingsModifier,
    artifacts_modifier: ArtifactsModifier,
    monsters_modifier: MonstersModifier,
}

impl ModifiersQueue {

    pub fn new() -> Self {
        ModifiersQueue { 
            primary_quests: vec![], 
            secondary_quests: vec![], 
            buildings_modifier: BuildingsModifier::new(), 
            artifacts_modifier: ArtifactsModifier::new(),
            monsters_modifier: MonstersModifier::new()
        }
    }

    pub fn apply_changes_to_map(&mut self, map: &Map, map_data: &mut MapData) {
        let mut output_map: Vec<u8> = Vec::new();
        let mut writer = Writer::new_with_indent(&mut output_map, b' ', 4);
    
        let map_string = std::fs::read_to_string(&map.xdb).unwrap();
    
        let mut reader = Reader::from_str(&map_string);
        let reader_config = reader.config_mut();
        reader_config.expand_empty_elements = true;
        reader_config.trim_text(true);
    
        let mut buf: Vec<u8> = Vec::new();
    
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None))).unwrap();
        let mut players_count = 0;
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    // gets actual name of tag
                    let actual_tag = std::str::from_utf8(e.name().as_ref()).unwrap().to_string();
                    match actual_tag.as_str() {
                        "AdvMapBuilding" => { 
                            let end = e.to_end().into_owned();
                            let text = reader.read_text(end.name()).unwrap().to_string();
                            println!("Building text: {}", &text);
                            let mut building: AdvMapBuilding = quick_xml::de::from_str(&format!("<AdvMapBuilding>{}</AdvMapBuilding>", &text)).unwrap();
                            self.buildings_modifier.modify(&mut building, &mut writer);
                        },
                        "AdvMapArtifact" => { 
                            let end = e.to_end().into_owned();
                            let text = reader.read_text(end.name()).unwrap().to_string();
                            let mut artifact: AdvMapArtifact = quick_xml::de::from_str(&format!("<AdvMapArtifact>{}</AdvMapArtifact>", &text)).unwrap();
                            self.artifacts_modifier.modify(&mut artifact, &mut writer);
                        },
                        "AdvMapMonster" => {
                            let end = e.to_end().into_owned();
                            let text = reader.read_text(end.name()).unwrap().to_string();
                            let mut monster: AdvMapMonster = quick_xml::de::from_str(&format!("<AdvMapMonster>{}</AdvMapMonster>", &text)).unwrap();
                            self.monsters_modifier.modify(&mut monster, &mut writer);
                        }
                        "Objectives" => {
                            println!("Objectives found");
                            let end = e.to_end().into_owned();
                            let text = reader.read_text(end.name()).unwrap().to_string();
                            let mut objectives: ObjectivesInfo = quick_xml::de::from_str(&format!("<Objectives>{}</Objectives>", &text)).unwrap();
                            self.apply_quests(&mut writer, &mut objectives);
                        },
                        "ReserveHeroes" => {
                            players_count += 1;
                            let end = e.to_end().into_owned();
                            reader.read_to_end(end.name()).unwrap();
                            if let Some(heroes) = map_data.reserve_heroes.get_mut(&players_count) {
                                writer.write_event(Event::Start(BytesStart::new("ReserveHeroes"))).unwrap();
                                let mut heroes_count = 0;
                                for hero in heroes {
                                    heroes_count += 1;
                                    if hero.army_slots.is_some() && hero.army_slots.as_ref().unwrap().army_slots.is_none() {
                                        hero.army_slots = None;
                                    }
                                    if hero.is_untransferable.is_some() && hero.is_untransferable.as_ref().unwrap().items.is_none() {
                                        hero.is_untransferable = None;
                                    }
                                    if hero.artifact_ids.is_some() && hero.artifact_ids.as_ref().unwrap().items.is_none() {
                                        hero.artifact_ids = None;
                                    }
                                    if hero.editable.skills.is_some() && hero.editable.skills.as_ref().unwrap().items.is_none() {
                                        hero.editable.skills = None;
                                    }
                                    if hero.editable.spellIDs.is_some() && hero.editable.spellIDs.as_ref().unwrap().items.is_none() {
                                        hero.editable.spellIDs = None;
                                    }
                                    if hero.editable.perkIDs.is_some() && hero.editable.perkIDs.as_ref().unwrap().items.is_none() {
                                        hero.editable.perkIDs = None;
                                    }
                                    if hero.editable.FavoriteEnemies.is_some() && hero.editable.FavoriteEnemies.as_ref().unwrap().items.is_none() {
                                        hero.editable.FavoriteEnemies = None;
                                    }
                                    writer.create_element("Item")
                                        .with_attributes(
                                            vec![
                                                ("href", "#n:inline(AdvMapHero)"),
                                                ("id", &format!("item_P{}_H{}", players_count, heroes_count))
                                            ]
                                        )
                                        .write_inner_content(|w| {
                                            w.write_serializable("AdvMapHero", hero).unwrap();
                                            Ok(())
                                        }).unwrap();
                                }
                                writer.write_event(Event::End(BytesEnd::new("ReserveHeroes"))).unwrap();
                            }
                        },
                        _=> {
                            let mut elem = BytesStart::new(str::from_utf8(e.name().0).unwrap());
                            elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                            writer.write_event(Event::Start(elem)).unwrap();
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                    writer.write_event(Event::Text(e)).unwrap();
                },
                Ok(Event::End(e)) => {
                    let elem = BytesEnd::new(str::from_utf8(e.name().0).unwrap());
                    writer.write_event(Event::End(elem)).unwrap();
                },
                _ => ()
            }
            buf.clear();
        }
        
        std::fs::remove_file(&map.xdb).unwrap();
        let mut file = std::fs::File::create(&map.xdb).unwrap();
        file.write_all(&output_map).unwrap();
        let buildinds_lua_data = self.buildings_modifier.convert_to_lua();
        let artifacts_lua_data = self.artifacts_modifier.convert_to_lua();
        let monsters_lua_data = self.monsters_modifier.convert_to_lua();
        let mut buildings_lua_file = std::fs::File::create(format!("{}{}", &map.data_path, "buildings_generated_data.lua")).unwrap();
        let mut artifacts_lua_file = std::fs::File::create(format!("{}{}", &map.data_path, "artifacts_generated_data.lua")).unwrap();
        let mut monsters_lua_file = std::fs::File::create(format!("{}{}", &map.data_path, "monsters_generated_data.lua")).unwrap();
        buildings_lua_file.write_all(buildinds_lua_data.as_bytes()).unwrap();
        artifacts_lua_file.write_all(artifacts_lua_data.as_bytes()).unwrap();
        monsters_lua_file.write_all(monsters_lua_data.as_bytes()).unwrap();
    }


    fn apply_quests(&self, writer: &mut Writer<&mut Vec<u8>>, objectives_data: &mut ObjectivesInfo) {
        let primary_quests_items = &mut objectives_data.primary.player_specific.items.as_mut().unwrap()[0];
        let secondary_quests_items = &mut objectives_data.secondary.player_specific.items.as_mut().unwrap()[0];

        if !self.primary_quests.is_empty() {
            if primary_quests_items.objectives.as_mut().unwrap().items.is_none() {
                primary_quests_items.objectives.as_mut().unwrap().items = Some(vec![]);
            }

            let quests_to_apply = self.primary_quests.iter().filter(|q| {
                !primary_quests_items.objectives.as_ref().unwrap().items.as_ref().unwrap().iter().any(|eq| eq.name == q.name)
            }).collect::<Vec<&Quest>>();

            println!("Primary quests to apply: {:?}", &quests_to_apply);

            for quest in quests_to_apply {
                primary_quests_items.objectives.as_mut().unwrap().items.as_mut().unwrap().push(quest.clone());
            }
        }

        if !self.secondary_quests.is_empty() {
            if secondary_quests_items.objectives.as_mut().unwrap().items.is_none() {
                secondary_quests_items.objectives.as_mut().unwrap().items = Some(vec![]);
            }

            let quests_to_apply = self.secondary_quests.iter().filter(|q| {
                !secondary_quests_items.objectives.as_ref().unwrap().items.as_ref().unwrap().iter().any(|eq| eq.name == q.name)
            }).collect::<Vec<&Quest>>();

            println!("Secondary quests to apply: {:?}", &quests_to_apply);

            for quest in quests_to_apply {
                secondary_quests_items.objectives.as_mut().unwrap().items.as_mut().unwrap().push(quest.clone());
            }
        }

        if objectives_data.primary.common.as_ref().unwrap().objectives.as_ref().unwrap().items.is_none() {
            objectives_data.primary.common.as_mut().unwrap().objectives = None;
        }

        for i in 0..8 {
            if objectives_data.primary.player_specific.items.as_ref().unwrap()[i].objectives.as_ref().unwrap().items.is_none() {
                objectives_data.primary.player_specific.items.as_mut().unwrap()[i].objectives = None;
            }
        }

        if objectives_data.secondary.common.as_ref().unwrap().objectives.as_ref().unwrap().items.is_none() {
            objectives_data.secondary.common.as_mut().unwrap().objectives = None;
        }

        for i in 0..8 {
            if objectives_data.secondary.player_specific.items.as_ref().unwrap()[i].objectives.as_ref().unwrap().items.is_none() {
                objectives_data.secondary.player_specific.items.as_mut().unwrap()[i].objectives = None;
            }
        } 

        writer.write_serializable("Objectives", objectives_data).unwrap();
    }
}

impl Default for ModifiersQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub id: u16,
    pub name: String,
    pub campaign: u8,
    pub mission: u8,
    pub xdb: String,
    pub data_path: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapData {
    pub reserve_heroes: HashMap<i32, Vec<AdvMapHero>>
}

impl MapData {
    pub fn read(map: &Map) -> Self {    
        let map_string = std::fs::read_to_string(&map.xdb).unwrap();
        let mut reader = Reader::from_str(&map_string);
        let reader_config = reader.config_mut();
        reader_config.expand_empty_elements = true;
        reader_config.trim_text(true);
        let mut buf: Vec<u8> = Vec::new();
        let mut players_count = 0;
        let mut map_data = MapData {reserve_heroes: HashMap::new()};
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => return map_data,
                Ok(Event::Start(e)) => {
                    // gets actual name of tag
                    let actual_tag = std::str::from_utf8(e.name().as_ref()).unwrap().to_string();
                    match actual_tag.as_str() {
                        "ReserveHeroes" => {
                            players_count += 1;
                            let end = e.to_end().into_owned();
                            let text = reader.read_text(end.name()).unwrap().to_string();
                            map_data.read_reserve_heroes(&text, players_count);
                        },
                        _=> {
                            let mut elem = BytesStart::new(str::from_utf8(e.name().0).unwrap());
                            elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                },
                Ok(Event::End(e)) => {
                    let elem = BytesEnd::new(str::from_utf8(e.name().0).unwrap());
                },
                _ => ()
            }
            buf.clear();
        }
    }

    fn read_reserve_heroes(&mut self, heroes_data: &str, player_number: i32) {
        let mut reader = Reader::from_str(heroes_data);
        let reader_config = reader.config_mut();
        reader_config.expand_empty_elements = true;
        reader_config.trim_text(true);
        let mut buf: Vec<u8> = Vec::new();
        self.reserve_heroes.insert(player_number, vec![]);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    // gets actual name of tag
                    let actual_tag = std::str::from_utf8(e.name().as_ref()).unwrap().to_string();
                    match actual_tag.as_str() {
                        "AdvMapHero" => {
                            let end = e.to_end().into_owned();
                            let text = reader.read_text(end.name()).unwrap().to_string();
                            let adv_map_hero: Result<AdvMapHero, quick_xml::DeError> = quick_xml::de::from_str(&format!("<AdvMapHero>{}</AdvMapHero>", &text));
                            match adv_map_hero {
                                Ok(hero) => {
                                    if let Some(heroes) = self.reserve_heroes.get_mut(&player_number) {
                                        heroes.push(hero);
                                    }
                                },
                                Err(de_error) => {
                                    println!("Error deserializing AdvMapHero object: {}", de_error.to_string())
                                }
                            }
                        },
                        _=> {
                            let mut elem = BytesStart::new(str::from_utf8(e.name().0).unwrap());
                            elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                },
                Ok(Event::End(e)) => {
                    let elem = BytesEnd::new(str::from_utf8(e.name().0).unwrap());
                },
                _ => ()
            }
            buf.clear();
        }
    }
}

