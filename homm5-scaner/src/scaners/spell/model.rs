#![allow(clippy::enum_variant_names)]

use std::str::FromStr;
use homm5_types::spell::SpellShared;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use sea_orm::{FromJsonQueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::{core::{ToJsonCompatibleString, ToLua}, prelude::ResourcesModel};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "spells")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub game_id: String,
    pub name_txt: String,
    pub name: String,
    pub desc_txt: String,
    pub desc: String,
    pub icon_xdb: String,
    pub cost: i32,
    pub level: i32,
    pub school: MagicSchoolType,
    pub is_aimed: bool,
    pub is_area: bool,
    pub resources_cost: ResourcesModel,
    pub damage_formula: SpellFormulaModel,
    pub duration_formula: SpellFormulaModel,
    pub unused_data: SpellUnusedDataModel
}

#[derive(
    Debug,
    DeriveActiveEnum,
    EnumIter,
    EnumString,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MagicSchoolType {
    #[sea_orm(string_value = "MAGIC_SCHOOL_NONE")]
    #[serde(rename = "MAGIC_SCHOOL_NONE")]
    #[strum(serialize = "MAGIC_SCHOOL_NONE")]
    None,
    #[sea_orm(string_value = "MAGIC_SCHOOL_SPECIAL")]
    #[serde(rename = "MAGIC_SCHOOL_SPECIAL")]
    #[strum(serialize = "MAGIC_SCHOOL_SPECIAL")]
    Special,
    #[sea_orm(string_value = "MAGIC_SCHOOL_LIGHT")]
    #[serde(rename = "MAGIC_SCHOOL_LIGHT")]
    #[strum(serialize = "MAGIC_SCHOOL_LIGHT")]
    Light,
    #[sea_orm(string_value = "MAGIC_SCHOOL_DARK")]
    #[serde(rename = "MAGIC_SCHOOL_DARK")]
    #[strum(serialize = "MAGIC_SCHOOL_DARK")]
    Dark,
    #[sea_orm(string_value = "MAGIC_SCHOOL_DESTRUCTIVE")]
    #[serde(rename = "MAGIC_SCHOOL_DESTRUCTIVE")]
    #[strum(serialize = "MAGIC_SCHOOL_DESTRUCTIVE")]
    Destructive,
    #[sea_orm(string_value = "MAGIC_SCHOOL_SUMMONING")]
    #[serde(rename = "MAGIC_SCHOOL_SUMMONING")]
    #[strum(serialize = "MAGIC_SCHOOL_SUMMONING")]
    Summoning,
    #[sea_orm(string_value = "MAGIC_SCHOOL_ADVENTURE")]
    #[serde(rename = "MAGIC_SCHOOL_ADVENTURE")]
    #[strum(serialize = "MAGIC_SCHOOL_ADVENTURE")]
    Adventure,
    #[sea_orm(string_value = "MAGIC_SCHOOL_RUNIC")]
    #[serde(rename = "MAGIC_SCHOOL_RUNIC")]
    #[strum(serialize = "MAGIC_SCHOOL_RUNIC")]
    Runic,
    #[sea_orm(string_value = "MAGIC_SCHOOL_WARCRIES")]
    #[serde(rename = "MAGIC_SCHOOL_WARCRIES")]
    #[strum(serialize = "MAGIC_SCHOOL_WARCRIES")]
    Warcries,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct SpellFormulaElementModel {
    pub base: OrderedFloat<f32>,
    pub per_power: OrderedFloat<f32>
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct SpellFormulaModel {
    pub elements: Vec<SpellFormulaElementModel>
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct SpellUnusedDataModel {
    pub effect_texture: Option<String>,
    pub spell_book_predictions: Option<Vec<String>>,
    pub combat_log_texts: Option<Vec<String>>,
    pub can_select_dead: bool,
    pub target: String,
    pub element: String,
    pub damage_is_elemental: Option<bool>,
    pub visuals: Option<Vec<String>>,
    pub preset_price: Option<i32>,
    pub available_for_presets: Option<bool>
}

impl ToJsonCompatibleString for MagicSchoolType {
    fn to_json_compatible_repr(&self) -> &str {
        match self {
            MagicSchoolType::Adventure => "Adventure",
            MagicSchoolType::Dark => "Dark",
            MagicSchoolType::None => "None",
            MagicSchoolType::Special => "Special",
            MagicSchoolType::Light => "Light",
            MagicSchoolType::Destructive => "Destructive",
            MagicSchoolType::Summoning => "Summoning",
            MagicSchoolType::Runic => "Runic",
            MagicSchoolType::Warcries => "Warcries",
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<SpellShared> for Model {
    fn from(value: SpellShared) -> Self {
        Model {
            id: Default::default(),
            game_id: Default::default(),
            name_txt: if let Some(ref file) = value.NameFileRef {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            name: Default::default(),
            desc_txt: if let Some(ref file) = value.LongDescriptionFileRef {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            desc: Default::default(),
            icon_xdb: if let Some(ref file) = value.Texture {
                file.href.clone().unwrap_or(String::new())
            } else {
                String::new()
            },
            cost: value.TrainedCost,
            level: value.Level,
            school: MagicSchoolType::from_str(&value.MagicSchool).unwrap_or(MagicSchoolType::None),
            is_aimed: value.IsAimed,
            is_area: value.IsAreaAttack,
            resources_cost: ResourcesModel {
                crystal: value.sSpellCost.Crystal,
                gem: value.sSpellCost.Gem,
                wood: value.sSpellCost.Wood,
                ore: value.sSpellCost.Ore,
                mercury: value.sSpellCost.Mercury,
                sulfur: value.sSpellCost.Sulfur,
                gold: value.sSpellCost.Gold
            },
            damage_formula: if let Some(formula) = value.damage {
                SpellFormulaModel { elements: if let Some(items) = formula.items {
                    items.iter().map(|e| SpellFormulaElementModel {
                        base: e.Base.into(),
                        per_power: e.PerPower.into()
                    }).collect_vec()
                } else {
                    vec![]
                }  }
            } else {
                SpellFormulaModel { elements: vec![] }
            },
            duration_formula: if let Some(formula) = value.duration {
                SpellFormulaModel { elements: if let Some(items) = formula.items {
                    items.iter().map(|e| SpellFormulaElementModel {
                        base: e.Base.into(),
                        per_power: e.PerPower.into()
                    }).collect_vec()
                } else {
                    vec![]
                }  }
            } else {
                SpellFormulaModel { elements: vec![] }
            },
            unused_data: SpellUnusedDataModel {
                available_for_presets: value.AvailableForPresets,
                effect_texture: if let Some(texture_ref) = value.EffectTexture {
                    texture_ref.href
                } else {
                    None
                },
                can_select_dead: value.CanSelectDead,
                combat_log_texts: if let Some(texts) = value.CombatLogTexts  {
                    if let Some(text_items) = texts.items {
                        if text_items.iter().any(|f| f.href.is_some()) {
                            Some(Vec::from_iter(text_items.iter().filter_map(|f| {
                                if f.href.is_none() {
                                    None
                                } else {
                                    f.href.clone()
                                }
                            })))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                },
                spell_book_predictions: if let Some(texts) = value.SpellBookPredictions  {
                    if let Some(text_items) = texts.items {
                        if text_items.iter().any(|f| f.href.is_some()) {
                            Some(Vec::from_iter(text_items.iter().filter_map(|f| {
                                if f.href.is_none() {
                                    None
                                } else {
                                    f.href.clone()
                                }
                            })))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                },
                damage_is_elemental: value.DamageIsElemental,
                target: value.Target,
                element: value.Element,
                visuals: if let Some(texts) = value.visuals  {
                    if let Some(text_items) = texts.items {
                        if text_items.iter().any(|f| f.href.is_some()) {
                            Some(Vec::from_iter(text_items.iter().filter_map(|f| {
                                if f.href.is_none() {
                                    None
                                } else {
                                    f.href.clone()
                                }
                            })))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                },
                preset_price: value.PresetPrice
            }
        }
    }
}

impl ToLua for Model {
    fn to_lua_string(&self) -> String {
        let is_aimed = if self.is_aimed { "1" } else { "nil" };
        let is_area = if self.is_area { "1" } else { "nil" };
        format!(
            "\t[{}] = {{
        name = \"{}\",
        desc = \"{}\",
        icon = \"{}\",
        school = {},
        level = {},
        is_aimed = {},
        is_area = {}
    }},\n",
            self.id,
            self.name_txt,
            self.desc_txt,
            self.icon_xdb,
            self.school,
            self.level,
            is_aimed,
            is_area
        )
    }
}
