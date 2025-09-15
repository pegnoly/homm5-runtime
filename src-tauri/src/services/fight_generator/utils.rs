use editor_tools::prelude::{ArmyGenerationRuleParam, ArmyGenerationStatParam, ArmyGenerationStatRule, ArmySlotStackCountGenerationMode, ArmySlotStackUnitGenerationMode, ArmyStatGenerationModel, AssetArmySlotModel, AssetGenerationType, DifficultyType};
use homm5_scaner::prelude::{CreatureDBModel, Town};
use itertools::Itertools;
use serde_json::Number;
use sheets_connector::{error::Error, prelude::ValueRange, utils::*};
use uuid::Uuid;

pub struct SheetToArmyAssetsConverter {
    asset_id: Uuid
}

impl SheetToArmyAssetsConverter {
    pub fn new(asset_id: Uuid) -> Self {
        SheetToArmyAssetsConverter { asset_id }
    }
}

impl FromSheetValueRange for SheetToArmyAssetsConverter {
    type Output = Vec<AssetArmySlotModel>;
    
    fn from_value_range(&self, values: ValueRange) -> Result<Self::Output, sheets_connector::error::Error> {
        let mut assets_count = 0;
        
        if let Some(values) = values.values {
            for data in values {
                println!("Values: {:#?}", &data);
            }
        }

        Ok(vec![])
    }
}

pub struct ArmySlotsConverter<'a> {
    pub sheet_name: &'a String,
    pub creatures_data: &'a Vec<CreatureDBModel>,
    pub stats_elements_data: &'a Vec<ArmyStatGenerationModel>
}

pub trait IntoSheetValidatedValue {
    fn into_value(&self) -> String;
}

impl IntoSheetValidatedValue for Town {
    fn into_value(&self) -> String {
        match self {
            Town::TownAcademy => String::from("Academy [5]"),
            Town::TownNoType => String::from("Neutral [8]"),
            Town::TownHeaven => String::from("Haven [0]"),
            Town::TownPreserve => String::from("Preserve [3]"),
            Town::TownDungeon => String::from("Dungeon [4]"),
            Town::TownNecromancy => String::from("Necropolis [2]"),
            Town::TownInferno => String::from("Inferno [1]"),
            Town::TownFortress => String::from("Fortress [6]"),
            Town::TownStronghold => String::from("Stronghold [7]")
        }
    }
}

impl IntoSheetValidatedValue for Vec<&ArmyStatGenerationModel> {
    fn into_value(&self) -> String {
        self.iter().map(|model| {
            model.stats.values.iter().map(|value| {
                match value {
                    ArmyGenerationStatParam::Initiative => if model.rule == ArmyGenerationStatRule::MaxBy { 
                        String::from("MaxBy initiative [3]") 
                    } else {
                        String::from("MinBy initiative [2]") 
                    },
                    ArmyGenerationStatParam::Speed => if model.rule == ArmyGenerationStatRule::MaxBy { 
                        String::from("MaxBy speed [5]") 
                    } else {
                        String::from("MinBy speed [4]") 
                    },
                    ArmyGenerationStatParam::Hitpoints => if model.rule == ArmyGenerationStatRule::MaxBy { 
                        String::from("MaxBy hp [1]") 
                    } else {
                        String::from("MinBy hp [0]") 
                    },
                    ArmyGenerationStatParam::Attack => if model.rule == ArmyGenerationStatRule::MaxBy { 
                        String::from("MaxBy attack [7]") 
                    } else {
                        String::from("MinBy attack [6]") 
                    },
                    ArmyGenerationStatParam::Defence => if model.rule == ArmyGenerationStatRule::MaxBy { 
                        String::from("MaxBy defence [9]") 
                    } else {
                        String::from("MinBy defence [8]") 
                    }
                }
            })
            .join(",")
        })
        .join(",")
    }
}

impl IntoSheetValidatedValue for ArmyGenerationRuleParam {
    fn into_value(&self) -> String {
        match self {
            ArmyGenerationRuleParam::UpgradeOnly => String::from("Only upgrade [1]"),
            ArmyGenerationRuleParam::Generatable => String::from("Only generatable [0]"),
            ArmyGenerationRuleParam::Shooter => String::from("Only shooter [2]"),
            ArmyGenerationRuleParam::Caster => String::from("Only caster [3]"),
        }
    }
}

impl IntoSheetsData<ValueRange> for ArmySlotsConverter<'_> {
    type Input = Vec<AssetArmySlotModel>;

    fn into_sheets_data(&self, source: Vec<AssetArmySlotModel>) -> Result<ValueRange, sheets_connector::error::Error> {
        let mut data: Vec<Vec<serde_json::Value>> = vec![];
        for army_slot in &source {
            let mut values = vec![];
            values.push(match army_slot.count_generation_mode {
                ArmySlotStackCountGenerationMode::PowerBased => serde_json::Value::String(String::from("Power based [0]")),
                ArmySlotStackCountGenerationMode::Raw => serde_json::Value::String(String::from("Raw [1]"))
            }); 
            values.push(match army_slot.power_based_generation_type {
                AssetGenerationType::Dynamic => serde_json::Value::String(String::from("Dynamic [1]")),
                AssetGenerationType::Static => serde_json::Value::String(String::from("Static [0]"))
            });

            if army_slot.count_generation_mode == ArmySlotStackCountGenerationMode::PowerBased {
                // base powers data
                if let Some(data) = army_slot.base_powers.data.get(&DifficultyType::Easy) {
                    values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Base powers of easy difficulty".to_string()))?));
                }

                if let Some(data) = army_slot.base_powers.data.get(&DifficultyType::Medium) {
                    values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Base powers of medium difficulty".to_string()))?));
                }

                if let Some(data) = army_slot.base_powers.data.get(&DifficultyType::Hard) {
                    values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Base powers of hard difficulty".to_string()))?));
                }

                if let Some(data) = army_slot.base_powers.data.get(&DifficultyType::Heroic) {
                    values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Base powers of heroic difficulty".to_string()))?));
                }
            } else {
                if let Some(data) = army_slot.concrete_count.data.get(&DifficultyType::Easy) {
                    values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Base powers of easy difficulty".to_string()))?));
                }

                if let Some(data) = army_slot.concrete_count.data.get(&DifficultyType::Medium) {
                    values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Base powers of medium difficulty".to_string()))?));
                }

                if let Some(data) = army_slot.concrete_count.data.get(&DifficultyType::Hard) {
                    values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Base powers of hard difficulty".to_string()))?));
                }

                if let Some(data) = army_slot.concrete_count.data.get(&DifficultyType::Heroic) {
                    values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Base powers of heroic difficulty".to_string()))?));
                }
            }

            // 2 empty cells between base powers and grow
            values.push(serde_json::Value::String(String::with_capacity(0)));
            values.push(serde_json::Value::String(String::with_capacity(0)));

            if let Some(data) = army_slot.powers_grow.data.get(&DifficultyType::Easy) {
                values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Powers grow of easy difficulty".to_string()))?));
            }

            if let Some(data) = army_slot.powers_grow.data.get(&DifficultyType::Medium) {
                values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Powers grow of medium difficulty".to_string()))?));
            }

            if let Some(data) = army_slot.powers_grow.data.get(&DifficultyType::Hard) {
                values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Powers grow of hard difficulty".to_string()))?));
            }

            if let Some(data) = army_slot.powers_grow.data.get(&DifficultyType::Heroic) {
                values.push(serde_json::Value::Number(Number::from_i128(*data as i128).ok_or(Error::UndefinedValue("Powers grow of heroic difficulty".to_string()))?));
            }

            values.push(serde_json::Value::String(String::from("")));

            values.push(match army_slot.type_generation_mode {
                ArmySlotStackUnitGenerationMode::TierSlotBased => serde_json::Value::String(String::from("Tier-slot based [0]")),
                ArmySlotStackUnitGenerationMode::ConcreteUnit => serde_json::Value::String(String::from("Concrete unit [1]")),
            });

            values.push(serde_json::Value::String(String::with_capacity(0)));
            values.push(serde_json::Value::String(String::with_capacity(0)));

            if !army_slot.towns.towns.is_empty() {
                values.push(serde_json::Value::String(army_slot.towns.towns.iter().map(|t| t.into_value()).join(",")));
            } else {
                values.push(serde_json::Value::String(String::with_capacity(0)));
            }

            if !army_slot.tiers.tiers.is_empty() {
                values.push(serde_json::Value::String(army_slot.tiers.tiers.iter().map(|t| format!("Tier {} [{}]", t, t - 1)).join(",")));
            } else {
                values.push(serde_json::Value::String(String::with_capacity(0)));
            }

            if !army_slot.generation_rule.params.is_empty() {
                values.push(serde_json::Value::String(army_slot.generation_rule.params.iter().map(|r| r.into_value()).join(",")));
            } else {
                values.push(serde_json::Value::String(String::with_capacity(0)));
            }
            
            let stat_elements = self.stats_elements_data.iter().filter(|m| m.stack_id == army_slot.id).collect_vec();
            values.push(serde_json::Value::String(stat_elements.into_value()));

            values.push(serde_json::Value::String(String::with_capacity(0)));
            values.push(serde_json::Value::String(String::with_capacity(0)));

            if !army_slot.concrete_creatures.ids.is_empty() {
                values.push(serde_json::Value::String(
                    army_slot.concrete_creatures.ids.iter().map(|id| {
                        let model = self.creatures_data.iter().find(|m| m.id == *id).unwrap();
                        format!("{} [{}]", if let Some(inner_name) = &model.inner_name {
                            let mut updated_name = inner_name.clone();
                            updated_name.drain(3..).collect::<String>()
                        } else {
                            let mut updated_name = model.name.clone();
                            updated_name.drain(3..).collect()
                        }, id)
                    }).join(",")));
            } else {
                values.push(serde_json::Value::String(String::with_capacity(0)));
            }

            data.push(values);
        }
        Ok(ValueRange { 
            major_dimension: Some(String::from("COLUMNS")), 
            range: Some(format!("{}!B2:{}", self.sheet_name, 
                match source.len() {
                    1 => "B24",
                    2 => "C24",
                    3 => "D24",
                    4 => "E24",
                    5 => "F24",
                    6 => "G24",
                    7 => "H24",
                    _ => unreachable!()
                })
            ), values: Some(data) })
    }
}