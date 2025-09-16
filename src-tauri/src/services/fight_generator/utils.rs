use std::collections::HashMap;

use editor_tools::prelude::{ArmyGenerationRuleParam, ArmyGenerationStatParam, ArmyGenerationStatRule, ArmySlotGenerationRule, ArmySlotStackCountGenerationMode, ArmySlotStackUnitGenerationMode, ArmyStatGenerationModel, AssetArmySlotModel, AssetGenerationType, CreatureIds, CreatureTiers, CreatureTowns, DifficultyMappedValue, DifficultyType};
use homm5_scaner::prelude::{CreatureDBModel, Town};
use itertools::Itertools;
use serde_json::Number;
use sheets_connector::{error::Error, prelude::ValueRange, utils::*};
use strum::IntoEnumIterator;
use uuid::Uuid;

pub struct SheetToArmyAssetsConverter {
    asset_id: Uuid
    // creatures_data: &'a Vec<CreatureDBModel>
}

impl SheetToArmyAssetsConverter {
    pub fn new(asset_id: Uuid) -> Self {
        SheetToArmyAssetsConverter { asset_id }
    }
}

impl FromSheetValueRange for SheetToArmyAssetsConverter {
    type Output = Vec<AssetArmySlotModel>;
    
    fn convert_from_value_range(&self, values: ValueRange) -> Result<Self::Output, sheets_connector::error::Error> {
        let mut stacks = vec![];
        if let Some(values) = values.values {
            for (index, data) in values.iter().enumerate() {
                // first element is a count generation rule
                let count_rule = if let Some(count_rule_data) = data.first() {
                    match count_rule_data {
                        serde_json::Value::String(data) => {
                            match data.as_str() {
                                "Power based [0]" => Ok(ArmySlotStackCountGenerationMode::PowerBased),
                                "Raw [1]" => Ok(ArmySlotStackCountGenerationMode::Raw),
                                _=> Err(Error::UndefinedValue("Sheet data conversion: stack count rule".to_string()))
                            }
                        },
                        _=> Err(Error::UndefinedValue("Sheet data conversion: stack count rule".to_string()))
                    }
                } else {
                    Err(Error::UndefinedValue("Sheet data conversion: stack count rule".to_string()))
                }?;
                // second is grow rule
                let grow_rule = if let Some(grow_rule_data) = data.get(1) {
                    match grow_rule_data {
                        serde_json::Value::String(data) => {
                            match data.as_str() {
                                "Static [0]" => Ok(AssetGenerationType::Static),
                                "Dynamic [1]" => Ok(AssetGenerationType::Dynamic),
                                _=> Err(Error::UndefinedValue("Sheet data conversion: stack count rule".to_string()))
                            }
                        },
                        _=> Err(Error::UndefinedValue("Sheet data conversion: stack count rule".to_string()))
                    }
                } else {
                    Err(Error::UndefinedValue("Sheet data conversion: stack count rule".to_string()))
                }?;

                let base_counts_data = if let Some(base_counts) = data.get(2..6) {
                    Ok(DifficultyMappedValue {
                        data: HashMap::from([
                            (DifficultyType::Easy, base_counts[0].as_str()
                                .ok_or(Error::UndefinedValue("Sheet data conversion: base counts".to_string()))?
                                .parse()?),
                            (DifficultyType::Medium, base_counts[1].as_str()
                                .ok_or(Error::UndefinedValue("Sheet data conversion: base counts".to_string()))?
                                .parse()?),
                            (DifficultyType::Hard, base_counts[2].as_str()
                                .ok_or(Error::UndefinedValue("Sheet data conversion: base counts".to_string()))?
                                .parse()?),
                            (DifficultyType::Heroic, base_counts[3].as_str()
                                .ok_or(Error::UndefinedValue("Sheet data conversion: base counts".to_string()))?
                                .parse()?),
                        ])
                    })
                } else {
                    Err(Error::UndefinedValue("Sheet data conversion: base counts".to_string()))
                }?;

                let grow_data = if let Some(grow_data) = data.get(8..12) {
                    Ok(DifficultyMappedValue {
                        data: HashMap::from([
                            (DifficultyType::Easy, grow_data[0].as_str()
                                .ok_or(Error::UndefinedValue("Sheet data conversion: counts grow".to_string()))?
                                .parse()?),
                            (DifficultyType::Medium, grow_data[1].as_str()
                                .ok_or(Error::UndefinedValue("Sheet data conversion: counts grow".to_string()))?
                                .parse()?),
                            (DifficultyType::Hard, grow_data[2].as_str()
                                .ok_or(Error::UndefinedValue("Sheet data conversion: counts grow".to_string()))?
                                .parse()?),
                            (DifficultyType::Heroic, grow_data[3].as_str()
                                .ok_or(Error::UndefinedValue("Sheet data conversion: counts grow".to_string()))?
                                .parse()?),
                        ])
                    })
                } else {
                    Err(Error::UndefinedValue("Sheet data conversion: counts grow".to_string()))
                }?;

                let unit_generation_rule = if let Some(gen_rule_data) = data.get(13) {
                    match gen_rule_data {
                        serde_json::Value::String(data) => {
                            match data.as_str() {
                                "Tier-slot based [0]" => Ok(ArmySlotStackUnitGenerationMode::TierSlotBased),
                                "Concrete unit [1]" => Ok(ArmySlotStackUnitGenerationMode::ConcreteUnit),
                                _=> Err(Error::UndefinedValue("Sheet data conversion: stack count rule".to_string()))
                            }
                        },
                        _=> Err(Error::UndefinedValue("Sheet data conversion: unit generation rule".to_string()))
                    }
                } else {
                    Err(Error::UndefinedValue("Sheet data conversion: unit generation rule".to_string()))
                }?;

                // println!("Unit generation rule: {}", unit_generation_rule);

                let towns = if let Some(towns_data) = data.get(16) {
                    match towns_data {
                        serde_json::Value::String(data) => {
                            if data.is_empty() {
                                Ok(CreatureTowns { towns: vec![] })
                            } else {
                                let towns_strings_list = data.split(",").collect_vec();
                                Ok(CreatureTowns {
                                    towns: Vec::from_iter(towns_strings_list.iter().map(|t| {
                                        match *t {
                                            "Haven [0]" => Town::TownHeaven,
                                            "Inferno [1]" => Town::TownInferno,
                                            "Necropolis [2]" => Town::TownNecromancy,
                                            "Preserve [3]" => Town::TownPreserve,
                                            "Dungeon [4]" => Town::TownDungeon,
                                            "Academy [5]" => Town::TownAcademy,
                                            "Fortress [6]" => Town::TownFortress,
                                            "Stronghold [7]" => Town::TownStronghold,
                                            "Neutral [8]" => Town::TownNoType,
                                            _=> unreachable!()    
                                        }
                                    }))
                                })
                            }
                        },
                        _=> Err(Error::UndefinedValue("Sheet data conversion: towns".to_string()))
                    }
                } else {
                    Err(Error::UndefinedValue("Sheet data conversion: towns".to_string()))
                }?;

                let tiers = if let Some(tiers_data) = data.get(17) {
                    match tiers_data {
                        serde_json::Value::String(data) => {
                            if data.is_empty() {
                                Ok(CreatureTiers { tiers: vec![] })
                            } else {
                                let tiers_strings_list = data.split(",").collect_vec();
                                Ok(CreatureTiers {
                                    tiers: Vec::from_iter(tiers_strings_list.iter().map(|t| {
                                        match *t {
                                            "Tier 1 [0]" => 1,
                                            "Tier 2 [1]" => 2,
                                            "Tier 3 [2]" => 3,
                                            "Tier 4 [3]" => 4,
                                            "Tier 5 [4]" => 5,
                                            "Tier 6 [5]" => 6,
                                            "Tier 7 [6]" => 7,
                                            _=> unreachable!()
                                        }
                                    }))
                                })
                            }
                        },
                        _=> Err(Error::UndefinedValue("Sheet data conversion: tiers".to_string()))
                    }
                } else {
                    Err(Error::UndefinedValue("Sheet data conversion: tiers".to_string()))
                }?;

                let params_rules = if let Some(params_rules_data) = data.get(18) {
                    match params_rules_data {
                        serde_json::Value::String(data) => {
                            if data.is_empty() {
                                Ok(ArmySlotGenerationRule { params: vec![] })
                            } else {
                                let params_strings_list = data.split(",").collect_vec();
                                Ok(ArmySlotGenerationRule {
                                    params: Vec::from_iter(params_strings_list.iter().map(|t| {
                                        match *t {
                                            "Only generatable [0]" => ArmyGenerationRuleParam::Generatable,
                                            "Only upgrade [1]" => ArmyGenerationRuleParam::UpgradeOnly,
                                            "Only shooter [2]" => ArmyGenerationRuleParam::Shooter,
                                            "Only caster [3]" => ArmyGenerationRuleParam::Caster,
                                            _=> unreachable!()
                                        }
                                    }))
                                })
                            }
                        },
                        _=> Err(Error::UndefinedValue("Sheet data conversion: params rules".to_string()))
                    }
                } else {
                    Err(Error::UndefinedValue("Sheet data conversion: params rules".to_string()))
                }?;

                let concrete_creatures = if let Some(creatures_data) = data.get(22) {
                    match creatures_data {
                        serde_json::Value::String(data) => {
                            if data.is_empty() {
                                Ok(CreatureIds { ids: vec![] })
                            } else {
                                let creatures_strings_list = data.split(",").collect_vec();
                                Ok(CreatureIds {
                                    ids: Vec::from_iter(creatures_strings_list.iter().filter_map(|c| {
                                        let parts: Vec<&str> = c.split('[').collect();
                                        if parts.len() > 1 {
                                            parts[1].split(']').next()
                                        } else {
                                            None
                                        }
                                    }).collect_vec().iter().filter_map(|c| c.parse::<i32>().ok()))
                                })
                            }
                        },
                        _=> Err(Error::UndefinedValue("Sheet data conversion: concrete creatures".to_string()))
                    }
                } else {
                    Err(Error::UndefinedValue("Sheet data conversion: concrete creatures".to_string()))
                }?;

                let stack = AssetArmySlotModel {
                    asset_id: self.asset_id,
                    id: 0,
                    number: index as i32,
                    type_generation_mode: unit_generation_rule,
                    count_generation_mode: count_rule.clone(),
                    power_based_generation_type: grow_rule,
                    base_powers: if count_rule == ArmySlotStackCountGenerationMode::PowerBased {
                        base_counts_data.clone()
                    } else {
                        DifficultyMappedValue {
                            data: HashMap::from_iter(DifficultyType::iter().map(|d| (d, 0)))
                        }
                    },
                    powers_grow: grow_data,
                    towns,
                    tiers,
                    generation_rule: params_rules,
                    concrete_creatures,
                    concrete_count: if count_rule == ArmySlotStackCountGenerationMode::Raw {
                        base_counts_data
                    } else {
                        DifficultyMappedValue {
                            data: HashMap::from_iter(DifficultyType::iter().map(|d| (d, 0)))
                        }
                    }, 
                };
                stacks.push(stack);
            }
        };
        Ok(stacks)
    }
}

pub struct ArmySlotsConverter<'a> {
    pub sheet_name: &'a String,
    pub creatures_data: &'a Vec<CreatureDBModel>,
    pub stats_elements_data: &'a Vec<ArmyStatGenerationModel>
}

pub trait IntoSheetValidatedValue {
    fn to_sheet_validated_value(&self) -> String;
}

impl IntoSheetValidatedValue for Town {
    fn to_sheet_validated_value(&self) -> String {
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
    fn to_sheet_validated_value(&self) -> String {
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
    fn to_sheet_validated_value(&self) -> String {
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

    fn convert_into_sheets_data(&self, source: Vec<AssetArmySlotModel>) -> Result<ValueRange, sheets_connector::error::Error> {
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
                values.push(serde_json::Value::String(army_slot.towns.towns.iter().map(|t| t.to_sheet_validated_value()).join(",")));
            } else {
                values.push(serde_json::Value::String(String::with_capacity(0)));
            }

            if !army_slot.tiers.tiers.is_empty() {
                values.push(serde_json::Value::String(army_slot.tiers.tiers.iter().map(|t| format!("Tier {} [{}]", t, t - 1)).join(",")));
            } else {
                values.push(serde_json::Value::String(String::with_capacity(0)));
            }

            if !army_slot.generation_rule.params.is_empty() {
                values.push(serde_json::Value::String(army_slot.generation_rule.params.iter().map(|r| r.to_sheet_validated_value()).join(",")));
            } else {
                values.push(serde_json::Value::String(String::with_capacity(0)));
            }

            let stat_elements = self.stats_elements_data.iter().filter(|m| m.stack_id == army_slot.id).collect_vec();
            values.push(serde_json::Value::String(stat_elements.to_sheet_validated_value()));

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