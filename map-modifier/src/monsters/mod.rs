use std::{collections::HashMap, sync::LazyLock};

use homm5_types::{common::FileRef, creature::AdvMapMonster};
use quick_xml::Writer;

static RANDOM_STACKS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        "/MapObjects/Random/Random-Monster-L1.(AdvMapMonsterShared).xdb#xpointer(/AdvMapMonsterShared)",
        "/MapObjects/Random/Random-Monster-L2.(AdvMapMonsterShared).xdb#xpointer(/AdvMapMonsterShared)",
        "/MapObjects/Random/Random-Monster-L3.(AdvMapMonsterShared).xdb#xpointer(/AdvMapMonsterShared)",
        "/MapObjects/Random/Random-Monster-L4.(AdvMapMonsterShared).xdb#xpointer(/AdvMapMonsterShared)",
        "/MapObjects/Random/Random-Monster-L5.(AdvMapMonsterShared).xdb#xpointer(/AdvMapMonsterShared)",
        "/MapObjects/Random/Random-Monster-L6.(AdvMapMonsterShared).xdb#xpointer(/AdvMapMonsterShared)",
        "/MapObjects/Random/Random-Monster-L7.(AdvMapMonsterShared).xdb#xpointer(/AdvMapMonsterShared)"
    ]
});

#[derive(Debug)]
pub struct RandomStackModel {
    pub is_runtime_generated: bool,
    pub min_stacks: u8,
    pub max_stacks: u8,
    pub total_power: u32
}

pub struct MonstersModifier {
    random_stacks_count: u32,
    random_stacks: HashMap<String, RandomStackModel>
}

impl MonstersModifier {
    pub fn new() -> Self {
        MonstersModifier { random_stacks_count: 0, random_stacks: HashMap::new() }
    }

    pub fn modify(&mut self, monster: &mut AdvMapMonster, writer: &mut Writer<&mut Vec<u8>>) {
        let empty = String::new();
        let default_file_ref = FileRef::default();
        let shared = monster.shared.as_ref().unwrap_or(&default_file_ref).href.as_ref().unwrap_or(&empty).as_str();
        if RANDOM_STACKS.iter().any(|rs| *rs == shared) && monster.runtime_generated {
            self.random_stacks_count += 1;
            let name = format!("GENERATABLE_STACK_{}", self.random_stacks_count);
            monster.name = Some(name.clone());
            self.random_stacks.insert(name.clone(), RandomStackModel { 
                is_runtime_generated: true, 
                min_stacks: monster.generated_stacks_min, 
                max_stacks: monster.generated_stacks_max, 
                total_power: monster.total_power 
            });
        }
        if monster.additional_stacks.is_some() && monster.additional_stacks.as_ref().unwrap().items.is_none() {
            monster.additional_stacks = None;
        }
        if monster.point_lights.is_some() && monster.point_lights.as_ref().unwrap().items.is_none() {
            monster.point_lights = None;
        }
        writer.write_serializable("AdvMapMonster", monster).unwrap();
    }

    pub fn convert_to_lua(&self) -> String {
        let mut lua_string = String::from("GENERATABLE_STACKS = {\n");
        for (name, model) in &self.random_stacks {
            lua_string += &format!("\t[\"{}\"] = {{min_stacks = {}, max_stacks = {}, power = {}}},\n", name, model.min_stacks, model.max_stacks, model.total_power);
        }
        lua_string.push_str("}\n\n");
        lua_string
    }
}