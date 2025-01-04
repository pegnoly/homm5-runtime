use core::str;
use std::io::Write;

pub use homm5_types::{common::FileRef, quest::{Objectives, Quest, QuestList}, Homm5Type};
use quick_xml::{events::{BytesDecl, BytesEnd, BytesStart, Event}, Reader, Writer};
use serde::{Deserialize, Serialize};

pub mod quest;

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
    pub secondary_quests: Vec<Quest>
}

impl ModifiersQueue {

    pub fn apply_changes_to_map(&self, map: &Map) {
        let mut output_map: Vec<u8> = Vec::new();
        let mut writer = Writer::new_with_indent(&mut output_map, b' ', 4);
    
        let map_string = std::fs::read_to_string(&map.xdb).unwrap();
    
        let mut reader = Reader::from_str(&map_string);
        let reader_config = reader.config_mut();
        reader_config.expand_empty_elements = true;
        reader_config.trim_text(true);
    
        let mut buf: Vec<u8> = Vec::new();
    
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None))).unwrap();
    
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    // gets actual name of tag
                    let actual_tag = std::str::from_utf8(e.name().as_ref()).unwrap().to_string();
                    if actual_tag == "Objectives" {
                        println!("Objectives found");
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name()).unwrap().to_string();
                        let mut objectives: ObjectivesInfo = quick_xml::de::from_str(&format!("<Objectives>{}</Objectives>", &text)).unwrap();
                        self.apply_quests(&mut writer, &mut objectives);
                    }
                    else {
                        let mut elem = BytesStart::new(str::from_utf8(e.name().0).unwrap());
                        elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                        writer.write_event(Event::Start(elem)).unwrap();
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
    }


    fn apply_quests(&self, writer: &mut Writer<&mut Vec<u8>>, objectives_data: &mut ObjectivesInfo) {
        let primary_quests_items = &mut objectives_data.primary.player_specific.items.as_mut().unwrap()[0];
        let secondary_quests_items = &mut objectives_data.secondary.player_specific.items.as_mut().unwrap()[0];

        if self.primary_quests.len() > 0 {
            if primary_quests_items.objectives.as_mut().unwrap().items.is_none() {
                primary_quests_items.objectives.as_mut().unwrap().items = Some(vec![]);
            }

            let quests_to_apply = self.primary_quests.iter().filter(|q| {
                primary_quests_items.objectives.as_ref().unwrap().items.as_ref().unwrap().iter().any(|eq| eq.name == q.name) == false
            }).collect::<Vec<&Quest>>();

            println!("Primary quests to apply: {:?}", &quests_to_apply);

            for quest in quests_to_apply {
                primary_quests_items.objectives.as_mut().unwrap().items.as_mut().unwrap().push(quest.clone());
            }
        }

        if self.secondary_quests.len() > 0 {
            if secondary_quests_items.objectives.as_mut().unwrap().items.is_none() {
                secondary_quests_items.objectives.as_mut().unwrap().items = Some(vec![]);
            }

            let quests_to_apply = self.secondary_quests.iter().filter(|q| {
                secondary_quests_items.objectives.as_ref().unwrap().items.as_ref().unwrap().iter().any(|eq| eq.name == q.name) == false
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub id: u16,
    pub name: String,
    pub campaign: u8,
    pub mission: u8,
    pub xdb: String,
    pub data_path: String
}