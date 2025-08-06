use homm5_scaner::prelude::{AbilityDBModel, CreatureDBModel};
use homm5_types::creature::{Abilities, Upgrades};
use quick_xml::{events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event}, Reader, Writer};

use crate::{error::Error, services::creature_generator::types::CreatableCreatureModel};

pub async fn process_generation(
    creatures_data: &[CreatureDBModel],
    abilities_data: &[AbilityDBModel],
    base_creature_model: &CreatureDBModel, 
    creature_model: &CreatableCreatureModel, 
    selected_abilities: &Vec<i32>
) -> Result<String, Error> {
    let mut output: Vec<u8> = Vec::new();
    let mut writer = Writer::new_with_indent(&mut output, b' ', 4);
    let mut reader = Reader::from_str(base_creature_model.xdb.as_ref().unwrap());
    let reader_config = reader.config_mut();
    reader_config.expand_empty_elements = true;
    reader_config.trim_text(true);

    let mut buf: Vec<u8> = Vec::new();

    let mut inner_name_created = false;

    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => {
                break Ok(String::from_utf8(output)?)
            },
            Ok(Event::Start(e)) => {
                let actual_tag = std::str::from_utf8(e.name().as_ref())?.to_string();
                match actual_tag.as_str() {
                    "BaseCreature" => {
                        let end = e.to_end().into_owned();
                        reader.read_to_end(end.name())?;
                        if creature_model.parent_creature.is_some() {
                            writer.create_element("BaseCreature").write_text_content(BytesText::new(
                    &creatures_data.iter()
                                .find(|cr| cr.id == creature_model.parent_creature.unwrap())
                                .unwrap()
                                .game_id
                            ))?;
                        } else {
                            writer.create_element("BaseCreature").write_text_content(BytesText::new("CREATURE_UNKNOWN"))?;
                        }
                    },
                    "PairCreature" => {
                        let end = e.to_end().into_owned();
                        reader.read_to_end(end.name())?;
                        writer.create_element("PairCreature").write_text_content(BytesText::new("CREATURE_UNKNOWN"))?;  
                    },
                    "SubjectOfRandomGeneration" => {
                        let end = e.to_end().into_owned();
                        reader.read_to_end(end.name())?;
                        writer.create_element("SubjectOfRandomGeneration").write_text_content(BytesText::new("false"))?; 
                    }
                    "Upgrades" => {
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name())?.to_string();
                        let mut upgrades = quick_xml::de::from_str::<Upgrades>(&format!("<Upgrades>{}</Upgrades>", &text))?;
                        if !creature_model.upgrades.is_empty() {
                            upgrades.upgrages = Some(
                                Vec::from_iter(creature_model.upgrades.iter()
                                    .map(|u| creatures_data.iter().find(|cr| cr.id == *u).unwrap().game_id.clone())));
                            writer.write_serializable("Upgrades", &upgrades)?;
                        } else {
                            writer.create_element("Upgrades").write_empty()?;
                        }
                    },
                    "Abilities" => {
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name())?.to_string();
                        let mut abilities = quick_xml::de::from_str::<Abilities>(&format!("<Abilities>{}</Abilities>", &text))?;
                        if !selected_abilities.is_empty() {
                            if abilities.Abilities.is_none() {
                                abilities.Abilities = Some(vec![]);
                            }
                            for ability in selected_abilities {
                                abilities.Abilities.as_mut().unwrap().push(abilities_data.iter().find(|a| a.id == *ability).unwrap().game_id.clone());
                            }
                        }
                        writer.write_serializable("Abilities", &abilities)?;
                    },
                    "MonsterShared" => {
                        let end = e.to_end().into_owned();
                        reader.read_to_end(end.name())?;
                        writer.create_element("MonsterShared").write_empty()?;  
                    },
                    _=> {
                        let mut elem = BytesStart::new(str::from_utf8(e.name().0)?);
                        elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                        writer.write_event(Event::Start(elem))?;
                        if !inner_name_created {
                            inner_name_created = true;
                            writer.create_element("InnerName").write_text_content(BytesText::new(creature_model.inner_name.as_ref().unwrap()))?;
                        }
                    }
                }
            }
            Ok(Event::Text(e)) => {
                writer.write_event(Event::Text(e))?;
            },
            Ok(Event::End(e)) => {
                let elem = BytesEnd::new(str::from_utf8(e.name().0)?);
                writer.write_event(Event::End(elem))?;
            },
            _ => ()
        }
        buf.clear();
    }
}