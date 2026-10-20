use crate::services::fight_generator::utils::IntoSheetValidatedValue;
use crate::services::version_control::{VersionControlContent, VersionControlService, VersionControlSimpleType};
use crate::utils::LocalAppManager;
use google_api::error::Error;
use google_api::prelude::ValueRange;
use google_api::utils::{FromSheetsData, IntoSheetsData};
use homm5_scaner::prelude::{
    AbilitiesModel, CreatureDBModel, FromSheetsCreatureUpdateModel, GetCreaturesRequest,
    KnownSpellsModel, MagicElement, MagicElementModel, Mastery, ResourcesModel, ScanerService,
    SpellWithMasteryModel, TownExtended, UpgradesModel,
};
use itertools::Itertools;
use regex::Regex;
use serde_json::Value;
use std::str::FromStr;
use std::vec;

pub struct CreaturesSheetsConverter;
pub struct CreatureModelSheetWrapper(CreatureDBModel);
impl Into<ValueRange> for CreatureModelSheetWrapper {
    fn into(self) -> ValueRange {
        let model = self.0;
        ValueRange {
            major_dimension: Some("ROWS".to_string()),
            range: Some(format!(
                "creatures_data!A{}:AH{}",
                model.id + 1,
                model.id + 1
            )),
            values: Some(vec![vec![
                Value::Number(model.id.into()),
                Value::String(model.name),
                if let Some(inner_name) = model.inner_name {
                    Value::String(inner_name)
                } else {
                    Value::String(String::new())
                },
                Value::String(model.town.to_sheet_validated_value()),
                Value::String(
                    model
                        .town_extended
                        .unwrap_or(TownExtended::TownNoType)
                        .to_sheet_validated_value(),
                ),
                Value::Number(model.attack.into()),
                Value::Number(model.defence.into()),
                Value::String(format!("{} - {}", model.min_damage, model.max_damage)),
                Value::Number(model.health.into()),
                Value::Number(model.speed.into()),
                Value::Number(model.initiative.into()),
                Value::Bool(model.is_flying),
                Value::Number(model.exp.into()),
                Value::Number(model.power.into()),
                Value::Number(model.tier.into()),
                Value::Number(model.grow.into()),
                Value::Number(model.size.into()),
                Value::Bool(model.is_generatable),
                Value::String(model.abilities.abilities.join("\n")),
                Value::String(
                    model
                        .known_spells
                        .spells
                        .iter()
                        .map(|s| {
                            format!(
                                "{} [{}]",
                                s.spell,
                                match s.mastery {
                                    Mastery::MasteryNone => "0",
                                    Mastery::MasteryBasic => "1",
                                    Mastery::MasteryAdvanced => "2",
                                    Mastery::MasteryExpert => "3",
                                    Mastery::MasteryExtraExpert => "4",
                                }
                            )
                        })
                        .join("\n"),
                ),
                Value::Number(model.spell_points.into()),
                Value::String(format!(
                    "{}/{}",
                    model.magic_element.first, model.magic_element.second
                )),
                Value::Number(model.range.into()),
                Value::Number(model.shots.into()),
                Value::String(format!(
                    "{}, {}, {}, {}, {}, {}, {}",
                    model.cost.gold,
                    model.cost.wood,
                    model.cost.ore,
                    model.cost.mercury,
                    model.cost.sulfur,
                    model.cost.crystal,
                    model.cost.gem
                )),
                Value::String(model.shared),
                Value::String(model.xdb_path),
                Value::String(model.name_txt),
                Value::String(model.desc_txt),
                Value::String(model.icon_xdb),
                Value::String(model.desc),
                Value::String(model.base_creature),
                Value::String(model.pair_creature),
                Value::String(model.upgrades.upgrades.join("\n")),
            ]]),
        }
    }
}

pub struct FromSheetCreatureUpdateWrapper(FromSheetsCreatureUpdateModel);

impl TryFrom<ValueRange> for FromSheetCreatureUpdateWrapper {
    type Error = Error;

    fn try_from(value: ValueRange) -> Result<Self, Self::Error> {
        let inner_number_regex = Regex::new(r"\[(\d+)]").unwrap();
        let data = value
            .values
            .and_then(|row| row.first().cloned())
            .ok_or_else(|| {
                Error::UndefinedValue("Creature data can't be read from sheet".to_string())
            })?;
        let mut damages = vec![];
        let Value::String(damage) = data.get(7).ok_or_else(|| {
            Error::UndefinedValue("Creature damage can't be read from sheet".to_string())
        })?
        else {
            return Err(Error::UndefinedValue(
                "Creature damage can't be read from sheet".to_string(),
            ));
        };

        let damage_split: Vec<_> = damage.split('-').map(str::trim).collect();
        damages.push(damage_split[0].parse::<i32>()?);
        damages.push(damage_split[1].parse::<i32>()?);
        let mut model = FromSheetsCreatureUpdateModel {
            id: data
                .first()
                .and_then(|v| match v {
                    Value::String(s) => Some(s.parse::<i32>().unwrap()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature id can't be read from sheet".to_string())
                })?,
            name: data
                .get(1)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature name can't be read from sheet".to_string())
                })?,
            inner_name: data.get(2).and_then(|v| match v {
                Value::String(s) => {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.clone())
                    }
                }
                _ => None,
            }),
            town: data
                .get(3)
                .and_then(|v| match v {
                    Value::String(s) => inner_number_regex
                        .captures(s)
                        .map(|caps| caps[1].parse::<i32>().unwrap().into()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature town can't be read from sheet".to_string())
                })?,
            town_extended: data
                .get(4)
                .and_then(|v| match v {
                    Value::String(s) => Some(
                        inner_number_regex
                            .captures(s)
                            .map(|caps| caps[1].parse::<i32>().unwrap().into()),
                    ),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue(
                        "Creature extended town can't be read from sheet".to_string(),
                    )
                })?,
            attack: data
                .get(5)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature attack can't be read from sheet".to_string())
                })?,
            defence: data
                .get(6)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature defence can't be read from sheet".to_string())
                })?,
            min_damage: damages[0],
            max_damage: damages[1],
            health: data
                .get(8)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature health can't be read from sheet".to_string())
                })?,
            speed: data
                .get(9)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature speed can't be read from sheet".to_string())
                })?,
            initiative: data
                .get(10)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue(
                        "Creature initiative can't be read from sheet".to_string(),
                    )
                })?,
            is_flying: data
                .get(11)
                .and_then(|v| match v {
                    Value::String(s) => s.to_lowercase().parse::<bool>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue(
                        "Creature is_flying param can't be read from sheet".to_string(),
                    )
                })?,
            exp: data
                .get(12)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature exp can't be read from sheet".to_string())
                })?,
            power: data
                .get(13)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature power can't be read from sheet".to_string())
                })?,
            tier: data
                .get(14)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature tier can't be read from sheet".to_string())
                })?,
            grow: data
                .get(15)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature grow can't be read from sheet".to_string())
                })?,
            size: data
                .get(16)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature size can't be read from sheet".to_string())
                })?,
            is_generatable: data
                .get(17)
                .and_then(|v| match v {
                    Value::String(s) => s.to_lowercase().parse::<bool>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature attack can't be read from sheet".to_string())
                })?,
            abilities: data
                .get(18)
                .and_then(|v| match v {
                    Value::String(abilities_data_string) => {
                        if abilities_data_string.is_empty() {
                            Some(AbilitiesModel { abilities: vec![] })
                        } else {
                            Some(AbilitiesModel {
                                abilities: abilities_data_string
                                    .split('\n')
                                    .map(|s| s.trim().to_string())
                                    .collect_vec(),
                            })
                        }
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature abilities can't be read from sheet".to_string())
                })?,
            known_spells: data
                .get(19)
                .and_then(|v| match v {
                    Value::String(spells_data_string) => {
                        if spells_data_string.is_empty() {
                            Some(KnownSpellsModel { spells: vec![] })
                        } else {
                            let spells = spells_data_string
                                .split('\n')
                                .map(|s| s.trim())
                                .collect_vec();
                            Some(KnownSpellsModel {
                                spells: spells
                                    .iter()
                                    .map(|s| {
                                        let split = s.split('[').collect_vec();
                                        SpellWithMasteryModel {
                                            spell: split[0].trim().to_string(),
                                            mastery: if let Some(caps) = inner_number_regex
                                                .captures(&format!("[{}", split[1]))
                                            {
                                                let num: i32 = caps[1].parse().unwrap();
                                                match num {
                                                    0 => Mastery::MasteryNone,
                                                    1 => Mastery::MasteryBasic,
                                                    2 => Mastery::MasteryAdvanced,
                                                    3 => Mastery::MasteryExpert,
                                                    4 => Mastery::MasteryExtraExpert,
                                                    _ => unreachable!(),
                                                }
                                            } else {
                                                Mastery::MasteryNone
                                            },
                                        }
                                    })
                                    .collect_vec(),
                            })
                        }
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature spells can't be read from sheet".to_string())
                })?,
            spell_points: data
                .get(20)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature mana can't be read from sheet".to_string())
                })?,
            magic_element: data
                .get(21)
                .and_then(|v| match v {
                    Value::String(magic_element_string) => {
                        let elements = magic_element_string.split('/').collect_vec();
                        Some(MagicElementModel {
                            first: MagicElement::from_str(elements[0]).unwrap(),
                            second: MagicElement::from_str(elements[1]).unwrap(),
                        })
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue(
                        "Creature magic elements can't be read from sheet".to_string(),
                    )
                })?,
            range: data
                .get(22)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature range can't be read from sheet".to_string())
                })?,
            shots: data
                .get(23)
                .and_then(|v| match v {
                    Value::String(s) => s.parse::<i32>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature shots can't be read from sheet".to_string())
                })?,
            cost: data
                .get(24)
                .and_then(|v| match v {
                    Value::String(cost_string) => {
                        let costs = cost_string.split(',').map(|s| s.trim()).collect_vec();
                        Some(ResourcesModel {
                            gold: costs[0].parse().unwrap(),
                            wood: costs[1].parse().unwrap(),
                            ore: costs[2].parse().unwrap(),
                            mercury: costs[3].parse().unwrap(),
                            sulfur: costs[4].parse().unwrap(),
                            crystal: costs[5].parse().unwrap(),
                            gem: costs[6].parse().unwrap(),
                        })
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature cost can't be read from sheet".to_string())
                })?,
            shared: data
                .get(25)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue(
                        "Creature shared path can't be read from sheet".to_string(),
                    )
                })?,
            xdb_path: data
                .get(26)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature xdb path can't be read from sheet".to_string())
                })?,
            name_txt: data
                .get(27)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature name path can't be read from sheet".to_string())
                })?,
            desc_txt: data
                .get(28)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature desc path can't be read from sheet".to_string())
                })?,
            icon_xdb: data
                .get(29)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature icon path can't be read from sheet".to_string())
                })?,
            desc: data
                .get(30)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature desc can't be read from sheet".to_string())
                })?,
            base_creature: data
                .get(31)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature base can't be read from sheet".to_string())
                })?,
            pair_creature: data
                .get(32)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature name can't be read from sheet".to_string())
                })?,
            upgrades: data
                .get(33)
                .and_then(|v| match v {
                    Value::String(upgrades_string) => {
                        if upgrades_string.is_empty() {
                            Some(UpgradesModel { upgrades: vec![] })
                        } else {
                            Some(UpgradesModel {
                                upgrades: upgrades_string
                                    .split('\n')
                                    .map(|s| s.trim().to_string())
                                    .collect_vec(),
                            })
                        }
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::UndefinedValue("Creature upgrades can't be read from sheet".to_string())
                })?,
            is_upgrade: false,
        };
        model.is_upgrade = model.base_creature != "CREATURE_UNKNOWN";
        Ok(FromSheetCreatureUpdateWrapper(model))
    }
}

impl FromSheetsData<FromSheetCreatureUpdateWrapper> for CreaturesSheetsConverter {}
impl IntoSheetsData<CreatureModelSheetWrapper> for CreaturesSheetsConverter {}

pub async fn push_creatures_data(
    app_manager: &LocalAppManager,
    version_control: &mut VersionControlService,
    scaner_service: &ScanerService,
) -> Result<Vec<i32>, crate::error::Error> {
    let profile = app_manager.current_profile_data.read().await;
    let updated_ids = version_control
        .updated_creatures
        .iter()
        .map(|id| *id)
        .collect_vec();

    let updated_models = scaner_service
        .get_creature_models(GetCreaturesRequest::OfIds(&updated_ids))
        .await?;
    let sheet_data = profile
        .global_sheets
        .get(&VersionControlSimpleType::Creatures)
        .ok_or(crate::error::Error::UndefinedData(
            "Can't find sheet data for creatures version control".to_string(),
        ))?;
    version_control
        .commit_many(
            &sheet_data.spreadsheet_id,
            updated_models.iter().map(|c| CreatureModelSheetWrapper(c.clone())).collect_vec(),
            CreaturesSheetsConverter {},
        )
        .await?;

    Ok(updated_ids)
}
