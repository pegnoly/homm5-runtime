use std::collections::HashMap;

use crate::{error::ScanerError, pak::FileStructure};

pub struct CreatureTypeItem {
    pub name: String,
    pub value: i32
}

pub struct TypesXmlScaner {
    pub creature_items: Vec<CreatureTypeItem>
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
        let elem = document.descendants()
            .find(|n| n.tag_name() == "TypeName".into() && n.text().unwrap() == "CreatureType")
            .unwrap()
            .parent()
            .unwrap();

        let entries_node = elem.descendants()
            .find(|n| n.tag_name() == "Entries".into())
            .unwrap();
        let mut name = String::new();
        let mut value = -1;
        entries_node.descendants().for_each(|n| {
            if n.tag_name() == "Name".into() {
                name = n.text().unwrap().to_string();
            } else if n.tag_name() == "Value".into() {
                value = n.text().unwrap().parse::<i32>().unwrap();
                self.creature_items.push(CreatureTypeItem { name: name.clone(), value: value });
            }
        });
        Ok(())
    }
}