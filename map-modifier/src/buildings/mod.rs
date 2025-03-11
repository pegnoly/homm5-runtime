use std::{collections::HashMap, sync::LazyLock};

use homm5_types::building::AdvMapBuilding;
use quick_xml::Writer;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display, Clone, EnumIter)]
pub(self) enum NewBuildingType {
    #[strum(serialize = "BUILDING_TYPE_WEAPONSMITH")]
    Weaponsmith,
    #[strum(serialize = "BUILDING_TYPE_ARMORSMITH")]
    Armorsmith,
    #[strum(serialize = "BUILDING_TYPE_JEWELRY")]
    Jewelry
}

static NEW_BUILDING_XDBS: LazyLock<HashMap<NewBuildingType, &str>> = LazyLock::new(|| {
        HashMap::from([
            (NewBuildingType::Weaponsmith, "/MapObjects/NewObjects/Weaponsmith.xdb#xpointer(/AdvMapBuildingShared)"),
            (NewBuildingType::Armorsmith, "/MapObjects/NewObjects/Armorsmith.xdb#xpointer(/AdvMapBuildingShared)"),
            (NewBuildingType::Jewelry, "/MapObjects/NewObjects/Armorsmith.xdb#xpointer(/AdvMapBuildingShared)")
        ])
    }
);

pub struct BuildingsModifier {
    new_buildings: HashMap<NewBuildingType, Vec<String>>,
    new_buildings_counts: HashMap<NewBuildingType, u32>
}

impl BuildingsModifier {
    pub fn new() -> Self {
        BuildingsModifier { new_buildings: HashMap::new(), new_buildings_counts: HashMap::new() }
    }

    pub fn modify(&mut self, building: &mut AdvMapBuilding, writer: &mut Writer<&mut Vec<u8>>) {
        let empty = String::new();
        let building_shared = building.shared.href.as_ref().unwrap_or(&empty);
        if !building_shared.is_empty() {
            if let Some(building_data) = NEW_BUILDING_XDBS.iter().find(|(_type, shared)| { **shared == building_shared.as_str() } ) {
                let possible_current_count = self.new_buildings_counts.get_mut(building_data.0);
                let mut building_count = if possible_current_count.is_some() { *possible_current_count.unwrap() } else { 0 };
                building_count += 1;
                building.name = format!("{}_{}", &building_data.0.to_string(), building_count);
                self.new_buildings_counts.insert(building_data.0.clone(), building_count);
                if let Some(buildings) = self.new_buildings.get_mut(building_data.0) {
                    buildings.push(building.name.clone());
                } else {
                    self.new_buildings.insert(building_data.0.clone(), vec![building.name.clone()]);
                }
            }
        }
        if building.point_lights.is_some() && building.point_lights.as_ref().unwrap().items.is_none() {
            building.point_lights = None;
        }
        writer.write_serializable("AdvMapBuilding", building).unwrap();
    }

    pub fn convert_to_lua(&self) -> String {
        let mut lua_code = String::from("NEW_BUILDINGS_DATA = {\n");
        NewBuildingType::iter().for_each(|_type| {
            if let Some(buildings) = self.new_buildings.get(&_type) {
                for building in buildings {
                    lua_code += &format!("\t[\"{}\"] = {},\n", building, _type.to_string())
                }
            }
        });
        lua_code.push_str("}\n\n");
        lua_code
    }
}