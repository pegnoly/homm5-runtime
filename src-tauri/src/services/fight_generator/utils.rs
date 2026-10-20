use crate::services::version_control::{get_sheet_name, VersionControlService};
use editor_tools::prelude::{AssetArmySlotModel, AssetArtifactsModel, AssetUpdateType, CalculationRule, DifficultyMappedValue, DifficultyType, FightGeneratorRepo, GetAssetsRequest, OptionalArtifacts, RequiredArtifacts, SelectionType, StackGenerationRule, StackStatsRuleModel, StatParam, StatSelectionRule};
use google_api::prelude::ValueRange;
use google_api::utils::{FromSheetsData, IntoSheetsData};
use homm5_scaner::prelude::{ArtifactDBModel, ArtifactSlotType, CreatureDBModel, GetArtifactsRequest, GetCreaturesRequest, ScanerService, Town, TownExtended};
use itertools::Itertools;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use crate::error::Error;
use crate::utils::LocalAppManager;

pub trait IntoSheetValidatedValue {
    fn to_sheet_validated_value(&self) -> String;
}

impl IntoSheetValidatedValue for TownExtended {
    fn to_sheet_validated_value(&self) -> String {
        match self {
            TownExtended::TownAcademy => String::from("Академия [5]"),
            TownExtended::TownNoType => String::from("Нейтрал [-1]"),
            TownExtended::TownHeaven => String::from("Орден порядка [0]"),
            TownExtended::TownPreserve => String::from("Лесной союз [3]"),
            TownExtended::TownDungeon => String::from("Лига теней [4]"),
            TownExtended::TownNecromancy => String::from("Некрополис [2]"),
            TownExtended::TownInferno => String::from("Инферно [1]"),
            TownExtended::TownFortress => String::from("Северные кланы [6]"),
            TownExtended::TownStronghold => String::from("Великая орда [7]"),
            TownExtended::TownBastion => String::from("Твердыня [8]"),
            TownExtended::TownSanctuary => String::from("Святилище [9]"),
            TownExtended::TownRenegades => String::from("Ренегаты [10]"),
        }
    }
}

impl IntoSheetValidatedValue for Town {
    fn to_sheet_validated_value(&self) -> String {
        match self {
            Town::TownNoType => String::from("Нейтрал [-1]"),
            Town::TownHeaven => String::from("Орден порядка [0]"),
            Town::TownPreserve => String::from("Лесной союз [3]"),
            Town::TownAcademy => String::from("Академия [5]"),
            Town::TownDungeon => String::from("Лига теней [4]"),
            Town::TownNecromancy => String::from("Некрополис [2]"),
            Town::TownInferno => String::from("Инферно [1]"),
            Town::TownFortress => String::from("Северные кланы [6]"),
            Town::TownStronghold => String::from("Великая Орда [7]"),
            Town::TownSpecial => String::from("Специальный [8]"),
        }
    }
}

pub struct FightAssetArmySheetConverter;

impl IntoSheetsData<AssetArmySlotSheetWrapper<'_>> for FightAssetArmySheetConverter {}
impl FromSheetsData<AssetArmySlotSheetWrapper<'_>> for FightAssetArmySheetConverter {}

impl IntoSheetValidatedValue for CalculationRule {
    fn to_sheet_validated_value(&self) -> String {
        match self {
            CalculationRule::Raw => "Raw [1]".to_string(),
            CalculationRule::PowerBased => "Power based [0]".to_string(),
        }
    }
}

impl IntoSheetValidatedValue for SelectionType {
    fn to_sheet_validated_value(&self) -> String {
        match self {
            SelectionType::Manual => "Specific unit [1]".to_string(),
            SelectionType::TierSlotBased => "Tier-slot based [0]".to_string(),
        }
    }
}

impl IntoSheetValidatedValue for AssetUpdateType {
    fn to_sheet_validated_value(&self) -> String {
        match self {
            AssetUpdateType::Static => "Static [0]".to_string(),
            AssetUpdateType::Dynamic => "Dynamic [1]".to_string(),
        }
    }
}

impl IntoSheetValidatedValue for StackGenerationRule {
    fn to_sheet_validated_value(&self) -> String {
        match self {
            StackGenerationRule::UpgradeOnly => "Only upgrade [1]".to_string(),
            StackGenerationRule::Generatable => "Only generatable [0]".to_string(),
            StackGenerationRule::Shooter => "Only shooter [2]".to_string(),
            StackGenerationRule::Caster => "Only caster [3]".to_string(),
        }
    }
}

impl IntoSheetValidatedValue for (StatSelectionRule, StatParam) {
    fn to_sheet_validated_value(&self) -> String {
        match (&self.0, &self.1) {
            (StatSelectionRule::MinBy, StatParam::Hitpoints) => "MinBy hp [0]".to_string(),
            (StatSelectionRule::MaxBy, StatParam::Hitpoints) => "MaxBy hp [1]".to_string(),
            (StatSelectionRule::MaxBy, StatParam::Initiative) => "MaxBy initiative [3]".to_string(),
            (StatSelectionRule::MaxBy, StatParam::Speed) => "MaxBy speed [5]".to_string(),
            (StatSelectionRule::MaxBy, StatParam::Attack) => "MaxBy attack [7]".to_string(),
            (StatSelectionRule::MaxBy, StatParam::Defence) => "MaxBy defence [9]".to_string(),
            (StatSelectionRule::MinBy, StatParam::Initiative) => "MinBy initiative [2]".to_string(),
            (StatSelectionRule::MinBy, StatParam::Speed) => "MinBy speed [4]".to_string(),
            (StatSelectionRule::MinBy, StatParam::Attack) => "MinBy attack [6]".to_string(),
            (StatSelectionRule::MinBy, StatParam::Defence) => "MinBy defence [8]".to_string(),
        }
    }
}

pub struct AssetArmySlotSheetWrapper<'a> {
    slot: AssetArmySlotModel,
    sheet_name: String,
    creatures_data: &'a [CreatureDBModel],
}

impl<'a> AssetArmySlotSheetWrapper<'a> {
    pub fn new(slot: AssetArmySlotModel) -> Self {
        Self {
            slot,
            sheet_name: String::new(),
            creatures_data: &[]
        }
    }
    fn get_column(&self) -> &'a str {
        match self.slot.number {
            0 => "B",
            1 => "C",
            2 => "D",
            3 => "E",
            4 => "F",
            5 => "G",
            6 => "H",
            _ => unreachable!(),
        }
    }
    fn convert_stack_base_power(&self, difficulty: DifficultyType) -> Value {
        if self.slot.calculation_type == CalculationRule::Raw {
            Value::Number(
                self.slot
                    .specific_count
                    .data
                    .get(&difficulty)
                    .copied()
                    .unwrap_or(0)
                    .into()
            )
        } else {
            Value::Number(
                self.slot
                    .base_powers
                    .data
                    .get(&difficulty)
                    .copied()
                    .unwrap_or(0)
                    .into(),
            )
        }
    }

    fn convert_stack_power_grow(&self, difficulty: DifficultyType) -> Value {
        Value::Number(
            self.slot
                .powers_grow
                .data
                .get(&difficulty)
                .copied()
                .unwrap_or(0)
                .into(),
        )
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<ValueRange> for AssetArmySlotSheetWrapper<'a> {
    fn into(self) -> ValueRange {
        let mut result = ValueRange::default();
        let col = self.get_column();
        result.range = Some(format!("{}!{}2:{}24", self.sheet_name, col, col));
        result.major_dimension = Some("COLUMNS".to_string());
        let mut values = vec![];
        values.push(Value::String(
            self.slot.calculation_type.to_sheet_validated_value(),
        ));
        values.push(Value::String(
            self.slot.update_type.to_sheet_validated_value(),
        ));
        values.push(self.convert_stack_base_power(DifficultyType::Easy));
        values.push(self.convert_stack_base_power(DifficultyType::Medium));
        values.push(self.convert_stack_base_power(DifficultyType::Hard));
        values.push(self.convert_stack_base_power(DifficultyType::Heroic));
        values.push(Value::String(String::new()));
        values.push(Value::String(String::new()));
        values.push(self.convert_stack_power_grow(DifficultyType::Easy));
        values.push(self.convert_stack_power_grow(DifficultyType::Medium));
        values.push(self.convert_stack_power_grow(DifficultyType::Hard));
        values.push(self.convert_stack_power_grow(DifficultyType::Heroic));
        values.push(Value::String(String::new()));
        values.push(Value::String(
            self.slot.selection_type.to_sheet_validated_value(),
        ));
        values.push(Value::String(String::new()));
        values.push(Value::String(String::new()));
        values.push(if self.slot.selection_type == SelectionType::Manual {
            Value::String(String::new())
        } else {
            Value::String(
                self.slot
                    .towns
                    .towns
                    .iter()
                    .map(|t| t.to_sheet_validated_value())
                    .collect_vec()
                    .join(","),
            )
        });
        values.push(if self.slot.selection_type == SelectionType::Manual {
            Value::String(String::new())
        } else {
            Value::String(
                self.slot
                    .tiers
                    .tiers
                    .iter()
                    .map(|t| format!("Tier {} [{}]", t, *t - 1))
                    .collect_vec()
                    .join(","),
            )
        });
        values.push(if self.slot.selection_type == SelectionType::Manual {
            Value::String(String::new())
        } else {
            Value::String(
                self.slot
                    .rules
                    .rules
                    .iter()
                    .map(|r| r.to_sheet_validated_value())
                    .collect_vec()
                    .join(","),
            )
        });
        values.push(if self.slot.selection_type == SelectionType::Manual {
            Value::String(String::new())
        } else {
            Value::String(
                self.slot.stat_rules.rules
                    .iter()
                    .map(|p| (p.rule.clone(), p.param.clone()).to_sheet_validated_value())
                    .collect_vec()
                    .join(","),
            )
        });
        values.push(Value::String(String::new()));
        values.push(Value::String(String::new()));
        values.push(if self.slot.selection_type == SelectionType::Manual {
            Value::String(
                self.slot
                    .specific_creatures
                    .ids
                    .iter()
                    .map(|c| {
                        let creature = self.creatures_data.iter().find(|m| m.id == *c).unwrap();
                        format!("{} [{}]", creature.name, c)
                    })
                    .collect_vec()
                    .join(","),
            )
        } else {
            Value::String(String::new())
        });
        result.values = Some(vec![values]);
        result
    }
}

impl<'a> TryFrom<ValueRange> for AssetArmySlotSheetWrapper<'a> {
    type Error = Error;
    fn try_from(value: ValueRange) -> Result<Self, Self::Error> {
        let inner_number_regex = Regex::new(r"\[(\d+)]").unwrap();
        let sheet_name = get_sheet_name(&value)?;
        let data = value
            .values
            .and_then(|v| v.first().cloned())
            .ok_or_else(|| Error::UndefinedData("Army slot sheet data".to_string()))?;

        let mut slot = AssetArmySlotModel::default();
        slot.sheet_name = sheet_name;
        slot.calculation_type = data
            .first()
            .and_then(|v| match v {
                Value::String(s) => {
                    inner_number_regex.captures(s).map(|caps| CalculationRule::from_repr(caps[1].parse().unwrap()).unwrap())
                }
                _ => None,
            })
            .ok_or_else(|| Error::UndefinedData("Army slot calc type".to_string()))?;
        slot.update_type = data
            .get(1)
            .and_then(|v| match v {
                Value::String(s) => {
                    inner_number_regex.captures(s).map(|caps| AssetUpdateType::from_repr(caps[1].parse().unwrap()).unwrap())
                }
                _ => None,
            })
            .ok_or_else(|| Error::UndefinedData("Army slot update type".to_string()))?;
        slot.selection_type = data
            .get(13)
            .and_then(|v| match v {
                Value::String(s) => {
                    inner_number_regex.captures(s).map(|caps| SelectionType::from_repr(caps[1].parse().unwrap()).unwrap())
                }
                _ => None,
            })
            .ok_or_else(|| Error::UndefinedData("Army slot selection type".to_string()))?;
        if slot.selection_type == SelectionType::Manual {
            slot.base_powers = DifficultyMappedValue::default();
            slot.specific_count = DifficultyMappedValue {
                data: HashMap::from([
                    (
                        DifficultyType::Easy,
                        data.get(2)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData(
                                    "Army slot specific count easy diff".to_string(),
                                )
                            })?,
                    ),
                    (
                        DifficultyType::Medium,
                        data.get(3)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData(
                                    "Army slot specific count medium diff".to_string(),
                                )
                            })?,
                    ),
                    (
                        DifficultyType::Hard,
                        data.get(4)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData(
                                    "Army slot specific count hard diff".to_string(),
                                )
                            })?,
                    ),
                    (
                        DifficultyType::Heroic,
                        data.get(5)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData(
                                    "Army slot specific count heroic diff".to_string(),
                                )
                            })?,
                    ),
                ]),
            };
            slot.specific_creatures.ids = data
                .last()
                .and_then(|v| match v {
                    Value::String(s) => Some(
                        s.split(",")
                            .filter_map(|c| {
                                inner_number_regex.captures(c).map(|caps| caps[1].parse().unwrap())
                            })
                            .collect_vec(),
                    ),
                    _ => None,
                })
                .ok_or_else(|| Error::UndefinedData("Army slot creatures ids".to_string()))?;
        } else {
            slot.specific_count = DifficultyMappedValue::default();
            slot.base_powers = DifficultyMappedValue {
                data: HashMap::from([
                    (
                        DifficultyType::Easy,
                        data.get(2)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData("Army slot base powers easy diff".to_string())
                            })?,
                    ),
                    (
                        DifficultyType::Medium,
                        data.get(3)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData(
                                    "Army slot base powers medium diff".to_string(),
                                )
                            })?,
                    ),
                    (
                        DifficultyType::Hard,
                        data.get(4)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData("Army slot base powers hard diff".to_string())
                            })?,
                    ),
                    (
                        DifficultyType::Heroic,
                        data.get(5)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData(
                                    "Army slot base powers heroic diff".to_string(),
                                )
                            })?,
                    ),
                ]),
            };
            slot.towns.towns = data
                .get(16)
                .and_then(|v| match v {
                    Value::String(s) => Some(
                        s.split(",")
                            .filter_map(|c| {
                                inner_number_regex.captures(c).map(|caps| TownExtended::from_repr(caps[1].parse().unwrap()).unwrap())
                            })
                            .collect_vec(),
                    ),
                    _ => None,
                })
                .ok_or_else(|| Error::UndefinedData("Army slot towns".to_string()))?;
            slot.tiers.tiers = data
                .get(17)
                .and_then(|v| match v {
                    Value::String(s) => Some(
                        s.split(",")
                            .filter_map(|c| {
                                inner_number_regex.captures(c).map(|caps| caps[1].parse().unwrap())
                            })
                            .collect_vec(),
                    ),
                    _ => None,
                })
                .ok_or_else(|| Error::UndefinedData("Army slot tiers".to_string()))?;
            slot.rules.rules = data
                .get(18)
                .and_then(|v| match v {
                    Value::String(s) => Some(
                        s.split(",")
                            .filter_map(|c| {
                                inner_number_regex.captures(c).map(|caps| StackGenerationRule::from_repr(caps[1].parse().unwrap())
                                            .unwrap())
                            })
                            .collect_vec(),
                    ),
                    _ => None,
                })
                .ok_or_else(|| Error::UndefinedData("Army slot rules".to_string()))?;
            slot.stat_rules.rules = data
                .get(19)
                .and_then(|v| match v {
                    Value::String(s) => {
                        if s.is_empty() {
                            None
                        } else {
                            Some(
                                s.split(",")
                                    .filter_map(|c| {
                                        if let Some(caps) = inner_number_regex.captures(c) {
                                            match caps[1].parse::<i32>().unwrap() {
                                                0 => Some(StackStatsRuleModel {
                                                    param: StatParam::Hitpoints,
                                                    rule: StatSelectionRule::MinBy,
                                                }),
                                                1 => Some(StackStatsRuleModel {
                                                    param: StatParam::Hitpoints,
                                                    rule: StatSelectionRule::MaxBy,
                                                }),
                                                2 => Some(StackStatsRuleModel {
                                                    param: StatParam::Initiative,
                                                    rule: StatSelectionRule::MinBy,
                                                }),
                                                3 => Some(StackStatsRuleModel {
                                                    param: StatParam::Initiative,
                                                    rule: StatSelectionRule::MaxBy,
                                                }),
                                                4 => Some(StackStatsRuleModel {
                                                    param: StatParam::Speed,
                                                    rule: StatSelectionRule::MinBy,
                                                }),
                                                5 => Some(StackStatsRuleModel {
                                                    param: StatParam::Speed,
                                                    rule: StatSelectionRule::MaxBy,
                                                }),
                                                6 => Some(StackStatsRuleModel {
                                                    param: StatParam::Attack,
                                                    rule: StatSelectionRule::MinBy,
                                                }),
                                                7 => Some(StackStatsRuleModel {
                                                    param: StatParam::Attack,
                                                    rule: StatSelectionRule::MaxBy,
                                                }),
                                                8 => Some(StackStatsRuleModel {
                                                    param: StatParam::Defence,
                                                    rule: StatSelectionRule::MinBy,
                                                }),
                                                9 => Some(StackStatsRuleModel {
                                                    param: StatParam::Defence,
                                                    rule: StatSelectionRule::MaxBy,
                                                }),
                                                _ => None,
                                            }
                                        } else {
                                            None
                                        }
                                    })
                                    .collect_vec(),
                            )
                        }
                    }
                    _ => None,
                })
                .ok_or_else(|| Error::UndefinedData("Army slot stats rules".to_string()))?;
        };
        slot.powers_grow = if slot.update_type == AssetUpdateType::Static {
            DifficultyMappedValue::default()
        } else {
            DifficultyMappedValue {
                data: HashMap::from([
                    (
                        DifficultyType::Easy,
                        data.get(8)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData("Army slot base powers easy diff".to_string())
                            })?,
                    ),
                    (
                        DifficultyType::Medium,
                        data.get(9)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData(
                                    "Army slot base powers medium diff".to_string(),
                                )
                            })?,
                    ),
                    (
                        DifficultyType::Hard,
                        data.get(10)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData("Army slot base powers hard diff".to_string())
                            })?,
                    ),
                    (
                        DifficultyType::Heroic,
                        data.get(11)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.parse().unwrap()),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                Error::UndefinedData(
                                    "Army slot base powers heroic diff".to_string(),
                                )
                            })?,
                    ),
                ]),
            }
        };

        Ok(AssetArmySlotSheetWrapper::new(slot))
    }
}

pub struct FightAssetArtifactsDataWrapper<'a> {
    data: AssetArtifactsModel,
    sheet_name: String,
    artifacts_data: &'a [ArtifactDBModel],
}

pub struct FightAssetArtifactsValueWrapper {
    data: AssetArtifactsModel,
    sheet_name: String,
}

#[derive(Clone, Copy)]
pub struct FightAssetArtifactsDataSheetConverter;

impl IntoSheetsData<FightAssetArtifactsDataWrapper<'_>> for FightAssetArtifactsDataSheetConverter {}
impl FromSheetsData<FightAssetArtifactsDataWrapper<'_>> for FightAssetArtifactsDataSheetConverter {}

impl IntoSheetsData<FightAssetArtifactsValueWrapper>
    for FightAssetArtifactsDataSheetConverter
{
}
impl FromSheetsData<FightAssetArtifactsValueWrapper>
    for FightAssetArtifactsDataSheetConverter
{
}

#[allow(clippy::from_over_into)]
impl<'a> Into<ValueRange> for FightAssetArtifactsDataWrapper<'a> {
    fn into(self) -> ValueRange {
        let mut result = ValueRange::default();
        result.range = Some(format!("{}!B27:B37", self.sheet_name));
        result.major_dimension = Some("COLUMNS".to_string());
        let mut values = vec![];
        values.push(Value::String(
            self.data
                .required
                .ids
                .iter()
                .map(|a| {
                    let art = self.artifacts_data.iter().find(|m| m.id == *a).unwrap();
                    format!("{} [{}]", art.name, art.id)
                })
                .collect_vec()
                .join(","),
        ));
        values.push(Value::String(String::new()));
        ArtifactSlotType::iter()
            .filter(|a| *a != ArtifactSlotType::Inventory)
            .for_each(|a| {
                if let Some(arts) = self.data.optional.values.get(&a) {
                    values.push(Value::String(
                        arts.iter()
                            .map(|i| {
                                let art = self.artifacts_data.iter().find(|d| d.id == *i).unwrap();
                                format!("{} [{}]", art.name, art.id)
                            })
                            .collect_vec()
                            .join(","),
                    ));
                } else {
                    values.push(Value::String(String::new()));
                }
            });
        result.values = Some(vec![values]);
        result
    }
}

impl<'a> TryFrom<ValueRange> for FightAssetArtifactsDataWrapper<'a> {
    type Error = Error;

    fn try_from(value: ValueRange) -> Result<Self, Self::Error> {
        let sheet_name = get_sheet_name(&value)?;
        let data = value
            .values
            .and_then(|v| v.first().cloned())
            .ok_or_else(|| Error::UndefinedData("Fight artifacts data".to_string()))?;
        let inner_number_regex = Regex::new(r"\[(\d+)]").unwrap();
        let model = FightAssetArtifactsDataWrapper {
            data: AssetArtifactsModel {
                sheet_name: sheet_name.clone(),
                id: -1,
                asset_id: -1,
                required: RequiredArtifacts {
                    ids: data
                        .first()
                        .and_then(|v| match v {
                            Value::String(s) => {
                                if s.is_empty() {
                                    Some(vec![])
                                } else {
                                    Some(
                                        s.split(",")
                                            .filter_map(|a| {
                                                inner_number_regex.captures(a).map(|caps| caps[1].parse().unwrap())
                                            })
                                            .collect_vec(),
                                    )
                                }
                            }
                            _ => None,
                        })
                        .ok_or_else(|| {
                            Error::UndefinedData("Artifacts asset required ids".to_string())
                        })?,
                },
                optional: OptionalArtifacts {
                    values: HashMap::from_iter(
                        ArtifactSlotType::iter()
                            .filter(|a| *a != ArtifactSlotType::Inventory)
                            .filter_map(|a| {
                                if let Some(v) = data.get(2 + (a.clone() as usize)) {
                                    match v {
                                        Value::String(s) => {
                                            if s.is_empty() {
                                                Some((a, vec![]))
                                            } else {
                                                Some((
                                                    a,
                                                    s.split(",")
                                                        .filter_map(|i| {
                                                            inner_number_regex.captures(i).map(|caps| caps[1].parse().unwrap())
                                                        })
                                                        .collect_vec(),
                                                ))
                                            }
                                        }
                                        _ => None,
                                    }
                                } else {
                                    None
                                }
                            }),
                    ),
                },
                powers_grow: DifficultyMappedValue::default(),
                base_powers: DifficultyMappedValue::default(),
                update_type: AssetUpdateType::default()
            },
            artifacts_data: &[],
            sheet_name: sheet_name.clone()
        };
        Ok(model)
    }
}

#[allow(clippy::from_over_into)]
impl Into<ValueRange> for FightAssetArtifactsValueWrapper {
    fn into(self) -> ValueRange {
        let mut result = ValueRange::default();
        result.range = Some(format!("{}!D29:D37", self.sheet_name));
        result.major_dimension = Some("COLUMNS".to_string());
        let mut values = vec![];

        DifficultyType::iter().for_each(|d| {
            values.push(Value::String(
                if let Some(data) = self.data.base_powers.data.get(&d) {
                    data.to_string()
                } else {
                    "0".to_string()
                },
            ));
        });

        values.push(Value::String(String::new()));

        DifficultyType::iter().for_each(|d| {
            values.push(Value::String(
                if let Some(data) = self.data.powers_grow.data.get(&d) {
                    data.to_string()
                } else {
                    "0".to_string()
                },
            ));
        });

        result.values = Some(vec![values]);
        result
    }
}

impl TryFrom<ValueRange> for FightAssetArtifactsValueWrapper {
    type Error = Error;

    fn try_from(value: ValueRange) -> Result<Self, Self::Error> {
        let sheet_name = get_sheet_name(&value)?;
        let data = value
            .values
            .and_then(|v| v.first().cloned())
            .ok_or_else(|| Error::UndefinedData("Fight artifacts values".to_string()))?;
        let model = FightAssetArtifactsValueWrapper {
            data: AssetArtifactsModel {
                sheet_name: sheet_name.clone(),
                base_powers: DifficultyMappedValue {
                    data: HashMap::from([
                        (
                            DifficultyType::Easy,
                            data.first()
                                .and_then(|v| match v {
                                    Value::String(s) => Some(s.parse().unwrap_or(0)),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    Error::UndefinedData(
                                        "Artifacts base powers easy diff".to_string(),
                                    )
                                })?,
                        ),
                        (
                            DifficultyType::Medium,
                            data.get(1)
                                .and_then(|v| match v {
                                    Value::String(s) => Some(s.parse().unwrap_or(0)),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    Error::UndefinedData(
                                        "Artifacts base powers medium diff".to_string(),
                                    )
                                })?,
                        ),
                        (
                            DifficultyType::Hard,
                            data.get(2)
                                .and_then(|v| match v {
                                    Value::String(s) => Some(s.parse().unwrap_or(0)),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    Error::UndefinedData(
                                        "Artifacts base powers hard diff".to_string(),
                                    )
                                })?,
                        ),
                        (
                            DifficultyType::Heroic,
                            data.get(3)
                                .and_then(|v| match v {
                                    Value::String(s) => Some(s.parse().unwrap_or(0)),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    Error::UndefinedData(
                                        "Artifacts base powers heroic diff".to_string(),
                                    )
                                })?,
                        ),
                    ]),
                },
                powers_grow: DifficultyMappedValue {
                    data: HashMap::from([
                        (
                            DifficultyType::Easy,
                            data.get(5)
                                .and_then(|v| match v {
                                    Value::String(s) => Some(s.parse().unwrap_or(0)),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    Error::UndefinedData(
                                        "Artifacts power grow easy diff".to_string(),
                                    )
                                })?,
                        ),
                        (
                            DifficultyType::Medium,
                            data.get(6)
                                .and_then(|v| match v {
                                    Value::String(s) => Some(s.parse().unwrap_or(0)),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    Error::UndefinedData(
                                        "Artifacts power grow medium diff".to_string(),
                                    )
                                })?,
                        ),
                        (
                            DifficultyType::Hard,
                            data.get(7)
                                .and_then(|v| match v {
                                    Value::String(s) => Some(s.parse().unwrap_or(0)),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    Error::UndefinedData(
                                        "Artifacts power grow hard diff".to_string(),
                                    )
                                })?,
                        ),
                        (
                            DifficultyType::Heroic,
                            data.get(8)
                                .and_then(|v| match v {
                                    Value::String(s) => Some(s.parse().unwrap_or(0)),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    Error::UndefinedData(
                                        "Artifacts power grow heroic diff".to_string(),
                                    )
                                })?,
                        ),
                    ]),
                },
                id: -1,
                asset_id: -1,
                update_type: AssetUpdateType::default(),
                required: RequiredArtifacts::default(),
                optional: OptionalArtifacts::default()
            },
            sheet_name: sheet_name.clone()
        };
        Ok(model)
    }
}

pub async fn commit_updated_fights(
    version_control: &mut VersionControlService,
    scaner_service: &ScanerService,
    app_manager: &LocalAppManager,
    fight_generator_repo: &FightGeneratorRepo,
) -> Result<HashMap<String, Vec<String>>, Error> {
    let creatures = scaner_service.get_creature_models(GetCreaturesRequest::NonDefault).await?;
    let artifacts = scaner_service.get_artifact_models(GetArtifactsRequest::Any).await?;
    let profile = app_manager.current_profile_data.read().await;
    let sheet_ids: HashMap<i32, String> = HashMap::from_iter(profile.maps.iter().map(|m| (m.id, m.fights_spreadsheet_id.clone())));
    let assets = fight_generator_repo
        .get_assets(GetAssetsRequest::OfIds(version_control.updated_fights.iter().map(|i| *i).collect())).await?;
    let mut commit_data: HashMap<String, Vec<String>> = HashMap::new();
    for asset in assets {
        let sheet_id = sheet_ids.get(&asset.mission_id).unwrap();
        if !commit_data.contains_key(sheet_id) {
            commit_data.insert(sheet_id.clone(),vec![]);
        }
        commit_data.get_mut(sheet_id).unwrap().push(asset.table_name.clone());
        let stacks = fight_generator_repo.get_stacks(asset.id).await?;
        let converter = FightAssetArmySheetConverter {};
        let stacks_wrappers = stacks.iter().map(|s| AssetArmySlotSheetWrapper {
            slot: s.clone(),
            sheet_name: asset.table_name.clone(),
            creatures_data: &creatures,
        }).collect_vec();

        version_control.commit_many(sheet_id, stacks_wrappers, converter).await?;

        if let Some(artifacts_model) = fight_generator_repo.get_artifacts_model(asset.id).await? {
            let converter = FightAssetArtifactsDataSheetConverter {};
            version_control.commit_single(sheet_id, FightAssetArtifactsDataWrapper {
                data: artifacts_model.clone(),
                sheet_name: asset.table_name.clone(),
                artifacts_data: &artifacts
            }, converter).await?;
            version_control.commit_single(sheet_id, FightAssetArtifactsValueWrapper {
                data: artifacts_model,
                sheet_name: asset.table_name,
            }, converter).await?;
        }
    }

    Ok(commit_data)
}

pub async fn pull_fights(){}