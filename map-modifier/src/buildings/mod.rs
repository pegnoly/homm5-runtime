use std::collections::HashMap;

use homm5_types::building::AdvMapBuilding;
use quick_xml::Writer;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display, Clone, EnumIter, Serialize, Deserialize)]
pub enum BuildingType {
    #[serde(rename = "BUILDING_TYPE_WEAPONSMITH")]
    #[strum(serialize = "BUILDING_TYPE_WEAPONSMITH")]
    Weaponsmith,
    #[serde(rename = "BUILDING_TYPE_ARMORSMITH")]   
    #[strum(serialize = "BUILDING_TYPE_ARMORSMITH")]
    Armorsmith,
    #[serde(rename = "BUILDING_TYPE_JEWELRY")]
    #[strum(serialize = "BUILDING_TYPE_JEWELRY")]
    Jewelry
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildingConfigEntity {
    #[serde(rename = "type")]
    pub _type: BuildingType,
    pub shared: String
}

#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display, Clone, EnumIter, Serialize, Deserialize)]
pub enum BankType {
    #[serde(rename = "BTD_BANK_CRYPT")]
    #[strum(to_string = "BTD_BANK_CRYPT")]
    Crypt,
    #[serde(rename = "BTD_BANK_PYRAMID")]
    #[strum(to_string = "BTD_BANK_PYRAMID")]
    Pyramid,
    #[serde(rename = "BTD_BANK_MAGI_VAULT")]
    #[strum(to_string = "BTD_BANK_MAGI_VAULT")]
    MagiVault,
    #[serde(rename = "BTD_BANK_DRAGON_UTOPIA")]
    #[strum(to_string = "BTD_BANK_DRAGON_UTOPIA")]
    DragonUtopia,
    #[serde(rename = "BTD_BANK_ELEMENTAL_STOCKPILE")]
    #[strum(to_string = "BTD_BANK_ELEMENTAL_STOCKPILE")]
    ElementalStockpile,
    #[serde(rename = "BTD_BANK_DWARVEN_TREASURE")]
    #[strum(to_string = "BTD_BANK_DWARVEN_TREASURE")]
    DwarvenTreasure,
    #[serde(rename = "BTD_BANK_BLOOD_TEMPLE")]
    #[strum(to_string = "BTD_BANK_BLOOD_TEMPLE")]
    BloodTemple,
    #[serde(rename = "BTD_BANK_TREANT_THICKET")]
    #[strum(to_string = "BTD_BANK_TREANT_THICKET")]
    TreantThicket,
    #[serde(rename = "BTD_BANK_GARGOYLE_STONEVAULT")]
    #[strum(to_string = "BTD_BANK_GARGOYLE_STONEVAULT")]
    GargoyleStonevault,
    #[serde(rename = "BTD_BANK_SUNKEN_TEMPLE")]
    #[strum(to_string = "BTD_BANK_SUNKEN_TEMPLE")]
    SunkenTemple
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BankConfigEntity {
    #[serde(rename = "type")]
    pub _type: BankType,
    pub shared: String
}

pub struct BuildingsModifier<'a> {
    banks_data: &'a Vec<BankConfigEntity>,
    buildings_data: &'a Vec<BuildingConfigEntity>,
    new_buildings: HashMap<BuildingType, Vec<String>>,
    new_banks: HashMap<BankType, Vec<String>>
}

impl<'a> BuildingsModifier<'a> {
    pub fn new(buildings_config_data: &'a Vec<BuildingConfigEntity>, banks_config_data: &'a Vec<BankConfigEntity>) -> Self {
        BuildingsModifier { 
            banks_data: banks_config_data,
            buildings_data: buildings_config_data,
            new_buildings: HashMap::new(), 
            new_banks: HashMap::new(),
        }
    }

    pub fn modify(&mut self, building: &mut AdvMapBuilding, writer: &mut Writer<&mut Vec<u8>>) {
        let empty = String::new();
        let building_shared = building.shared.href.as_ref().unwrap_or(&empty);
        if !building_shared.is_empty() {
            if let Some(building_data) = self.buildings_data.iter().find(|b| { b.shared == building_shared.as_str() } ) {
                if let Some(buildings) = self.new_buildings.get_mut(&building_data._type) {
                    building.name = format!("{}_{}", building_data._type, buildings.len() + 1);
                    buildings.push(building.name.clone());
                } else {
                    building.name = format!("{}_1", building_data._type);
                    self.new_buildings.insert(building_data._type.clone(), vec![building.name.clone()]);
                }
            }
            if let Some(bank_data) = self.banks_data.iter().find(|b| b.shared == *building_shared) {
                if let Some(banks) = self.new_banks.get_mut(&bank_data._type) {
                    building.name = format!("{}_{}", bank_data._type, banks.len() + 1);
                    banks.push(building.name.clone());
                } else {
                    building.name = format!("{}_1", bank_data._type);
                    self.new_banks.insert(bank_data._type.clone(), vec![building.name.clone()]);
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
        BuildingType::iter().for_each(|_type| {
            if let Some(buildings) = self.new_buildings.get(&_type) {
                for building in buildings {
                    lua_code += &format!("\t[\"{building}\"] = {_type},\n")
                }
            }
        });
        lua_code.push_str("}\n\nNEW_BANKS_DATA = {\n");
        BankType::iter().for_each(|_type| {
            if let Some(banks) = self.new_banks.get(&_type) {
                for bank in banks {
                    lua_code += &format!("\t[\"{bank}\"] = {_type},\n")
                }
            }
        });
        lua_code.push('}');
        lua_code
    }
}