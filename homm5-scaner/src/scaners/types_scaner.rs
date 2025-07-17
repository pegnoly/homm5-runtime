use std::collections::HashMap;

use crate::{error::ScanerError, pak::FileStructure};

pub struct GameTypeItem {
    pub name: String,
    pub value: i32
}

pub struct TypesXmlScaner {
    pub creature_items: Vec<GameTypeItem>,
    pub skills_items: Vec<GameTypeItem>,
    pub spells_items: Vec<GameTypeItem>
}

impl TypesXmlScaner {
    pub fn parse(        
        &mut self,
        files: &HashMap<String, FileStructure>
    ) -> Result<(), ScanerError> {
        let types_xml = files
            .iter()
            .find(|f| {
                f.0 == "types.xml"
                    .to_lowercase()
                    .as_str()
            })
            .unwrap();

        let document = roxmltree::Document::parse(&types_xml.1.content).unwrap();
        let creatures_elem = document.descendants()
            .find(|n| n.tag_name() == "TypeName".into() && n.text().unwrap() == "CreatureType")
            .unwrap()
            .parent()
            .unwrap();

        let creature_entries_node = creatures_elem.descendants()
            .find(|n| n.tag_name() == "Entries".into())
            .unwrap();
        let mut name = String::new();
        let mut value = -1;
        creature_entries_node.descendants().for_each(|n| {
            if n.tag_name() == "Name".into() {
                name = n.text().unwrap().to_string();
            } else if n.tag_name() == "Value".into() {
                value = n.text().unwrap().parse::<i32>().unwrap();
                self.creature_items.push(GameTypeItem { name: name.clone(), value: value });
            }
        });

        let skills_elem = document.descendants()
            .find(|n| n.tag_name() == "TypeName".into() && n.text().unwrap() == "SkillID")
            .unwrap()
            .parent()
            .unwrap();

        let skills_entries_node = skills_elem.descendants()
            .find(|n| n.tag_name() == "Entries".into())
            .unwrap();
        let mut name = String::new();
        let mut value = -1;
        skills_entries_node.descendants().for_each(|n| {
            if n.tag_name() == "Name".into() {
                name = n.text().unwrap().to_string();
            } else if n.tag_name() == "Value".into() {
                value = n.text().unwrap().parse::<i32>().unwrap();
                self.skills_items.push(GameTypeItem { name: name.clone(), value: value });
            }
        });

        let spell_elem = document.descendants()
            .find(|n| n.tag_name() == "TypeName".into() && n.text().unwrap() == "SpellID")
            .unwrap()
            .parent()
            .unwrap();

        let spells_entries_node = spell_elem.descendants()
            .find(|n| n.tag_name() == "Entries".into())
            .unwrap();
        let mut name = String::new();
        let mut value = -1;
        spells_entries_node.descendants().for_each(|n| {
            if n.tag_name() == "Name".into() {
                name = n.text().unwrap().to_string();
            } else if n.tag_name() == "Value".into() {
                value = n.text().unwrap().parse::<i32>().unwrap();
                self.spells_items.push(GameTypeItem { name: name.clone(), value: value });
            }
        });

        Ok(())
    }
}