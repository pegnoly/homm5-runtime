use super::{configure_path, CollectFiles, FileObjects, FileRef, FileStructure, Output, Scan};
use quick_xml::{Reader, events::Event};
use std::collections::HashMap;
use homm5_types::creature::{CreatureVisual, AdvMapCreatureShared};

pub struct CreatureFileCollector {}

impl CollectFiles for CreatureFileCollector {
    fn collect(&self, files: &HashMap<String, FileStructure>, collected_files: &mut Vec<(String, FileStructure)>){
        let creatures_xdb = files.iter()
            .find(|f| f.0 == "GameMechanics/RefTables/Creatures.xdb".to_lowercase().as_str())
            .unwrap();
        println!("creatures xdb pak - {}", &creatures_xdb.1.pak);
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(creatures_xdb.1.content.as_str());
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"objects" => {
                            let end = e.to_end().into_owned();
                            let text = reader.read_text(end.name()).unwrap().to_string();
                            let text = format!("<objects>{}</objects>", text);
                            let creatures_de: Result<FileObjects, quick_xml::DeError> = quick_xml::de::from_str(&text);
                            match creatures_de {
                                Ok(creatures) => {
                                    for creature in creatures.objects {
                                        match creature.Obj {
                                            Some(obj) => {
                                                let creature_key = obj.href.as_ref().unwrap()
                                                    .replace("#xpointer(/Creature)", "")
                                                    .trim_start_matches("/")
                                                    .to_lowercase();
                                                let creature_entity = files.get(&creature_key);
                                                match creature_entity {
                                                    Some(entity) => {
                                                        collected_files.push((creature_key.clone(), entity.clone()));
                                                    },
                                                    None => println!("Key {} is not in files", &creature_key)
                                                }
                                            }
                                            None => {}
                                        }
                                    }
                                },
                                Err(e) => println!("Error deserializing creatures.xdb, {}", e.to_string())
                            }
                        }
                        _=> {}
                    }
                }
                _ => ()
            }
            buf.clear();
        }
    }
}

// impl ToJson for AdvMapCreatureShared {
// }

impl Output for AdvMapCreatureShared {
    type ID = u16;
    fn to_lua(&self, id: Option<u16>) -> String {
        let is_generatable = if self.SubjectOfRandomGeneration == true {"1"} else {"nil"};
        let is_flying = if self.Flying == true {"1"} else {"nil"};
        let is_upgrade = if self.BaseCreature.is_some() && self.BaseCreature.as_ref().unwrap() != "CREATURE_UNKNOWN" { "1" } else { "nil "};
        let mut abilities_string = String::new();
        match &self.Abilities.Abilities {
            Some(abilities) => {
                for ability in abilities {
                    abilities_string += &format!("{}, ", &ability);
                }
            },
            None => {}
        }
        let mut spells_string = String::new();
        match &self.KnownSpells.spells {
            Some(spells) => {
                for spell in spells {
                    spells_string += &format!("[{}] = {}, ", &spell.Spell, &spell.Mastery);
                }
            },
            None => {}

        }
        format!(
            "\t[{}] = {{
        is_generatable = {},
        attack = {},
        defence = {},
        dmg_min = {},
        dmg_max = {},
        speed = {},
        ini = {},
        health = {},
        sp = {},
        size = {},
        exp = {},
        power = {},
        town = {},
        first_element = {},
        second_element = {},
        grow = {},
        tier = {},
        cost = {},
        range = {},
        name = \"{}\",
        desc = \"{}\",
        icon = \"{}\",
        is_flying = {},
        abilities = {{{}}},
        known_spells = {{{}}},
        is_upgrade = {}
    }},\n", 
            id.unwrap() - 1,
            is_generatable, 
            self.AttackSkill, 
            self.DefenceSkill, 
            self.MinDamage, 
            self.MaxDamage,
            self.Speed,
            self.Initiative,
            self.Health,
            self.SpellPoints,
            self.CombatSize,
            self.Exp,
            self.Power,
            self.CreatureTown,
            self.MagicElement.First,
            self.MagicElement.Second,
            self.WeeklyGrowth,
            self.CreatureTier,
            self.Cost.Gold,
            self.Range,
            self.VisualExplained.as_ref().unwrap().CreatureNameFileRef.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()),
            self.VisualExplained.as_ref().unwrap().DescriptionFileRef.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()),
            self.VisualExplained.as_ref().unwrap().Icon128.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()),
            is_flying,
            abilities_string,
            spells_string,
            is_upgrade
        )
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

pub struct CreatureScaner {
    pub id: u16
}

impl CreatureScaner {
    fn check_visual(&self, file_key: &String, content: &String, files: &HashMap<String, FileStructure>) -> Option<CreatureVisual> {
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(&content);
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break None,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"CreatureVisual" => {
                            let end = e.to_end().into_owned();
                            let possible_text = reader.read_text(end.name());
                            match possible_text {
                                Ok(text) => {
                                    let text = text.to_string();
                                    let de_res: Result<CreatureVisual, quick_xml::DeError> = quick_xml::de::from_str(&format!("<CreatureVisual>{}</CreatureVisual>", text));
                                    match de_res {
                                        Ok(visual) => {
                                            let name = configure_path(visual.CreatureNameFileRef.as_ref().unwrap().href.as_ref(), file_key, files);
                                            let desc = configure_path(visual.DescriptionFileRef.as_ref().unwrap().href.as_ref(), file_key, files);
                                            let icon_key = visual.Icon128.as_ref().unwrap().href.as_ref().unwrap_or(&String::new()).replace("#xpointer(/Texture)", "");
                                            let icon = configure_path(Some(&icon_key), file_key, files);
                                            break Some(CreatureVisual { 
                                                CreatureNameFileRef: Some(FileRef { href: Some(name) }), 
                                                DescriptionFileRef: Some(FileRef { href: Some(desc) }), 
                                                Icon128: Some(FileRef { href: Some(icon) }) 
                                            })
                                        }
                                        Err(e) => {
                                            println!("error while deserializing file key {}, {:?}", file_key, e.to_string());
                                        }
                                    }
                                },
                                Err(_e) => println!("error reading file content: {}", file_key)
                            }
                        }
                        _=> {}
                    }
                }
                _ => ()
            }
            buf.clear();
        }
    }
}

impl Scan<u16> for CreatureScaner {
    fn scan(&mut self, file_key: &String, entity: &String, files: &HashMap<String, FileStructure>) -> Option<Box<dyn Output<ID = u16>>> {
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(entity);
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break None,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"Creature" => {
                            let end = e.to_end().into_owned();
                            let possible_text = reader.read_text(end.name());
                            match possible_text {
                                Ok(text) => {
                                    let text = text.to_string();
                                    let de_res: Result<AdvMapCreatureShared, quick_xml::DeError> = quick_xml::de::from_str(
                                        &format!("<Creature>{}</Creature>", text)
                                    );
                                    self.id+=1;
                                    match de_res {
                                        Ok(mut creature) => {
                                            //println!("Creature scanned: {:?}", &creature);
                                            let visual = creature.Visual.as_ref();
                                            match visual {
                                                Some(actual_visual) => {
                                                    let visual_key = actual_visual.href.as_ref().unwrap()
                                                        .replace("#xpointer(/CreatureVisual)", "")
                                                        .trim_start_matches("/")
                                                        .to_lowercase();
                                                    println!("visual key: {}", &visual_key);
                                                    let actual_visual_key = configure_path(Some(&visual_key), file_key, files);
                                                    let visual_file = files.get(&actual_visual_key);
                                                    match visual_file {
                                                        Some(actual_visual_file) => {
                                                            let visual_checked = self.check_visual(&actual_visual_key, &actual_visual_file.content, files);
                                                            creature.VisualExplained = visual_checked;
                                                        },
                                                        None => println!("Can't find visual of {}", &actual_visual_key)
                                                    }
                                                },
                                                None => {}
                                            }
                                            //println!("Creature's visual: {:?}", &creature.VisualExplained);
                                            break Some(Box::new(creature));
                                        }
                                        Err(e) => {
                                            println!("error while deserializing file key {}, {:?}", file_key, e.to_string());
                                        }
                                    }
                                },
                                Err(_e) => println!("error reading file content: {}", file_key)
                            }
                        }
                        _=> {}
                    }
                }
                _ => ()
            }
            buf.clear();
        }
    }

    fn get_id(&self) -> Option<u16> {
        Some(self.id)
    }
}