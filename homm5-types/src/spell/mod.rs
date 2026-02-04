use crate::{common::FileRef, creature::Resources};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellBookPredictions {
    #[serde(rename = "Item")]
    pub items: Option<Vec<FileRef>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CombatLogTexts {
    #[serde(rename = "Item")]
    pub items: Option<Vec<FileRef>>
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SpellFormulaElement {
    pub Base: f32,
    pub PerPower: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellFormula {
    #[serde(rename = "Item")]
    pub items: Option<Vec<SpellFormulaElement>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Visuals {
    #[serde(rename = "Item")]
    pub items: Option<Vec<FileRef>>
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SpellShared {
    pub NameFileRef: Option<FileRef>,
    pub LongDescriptionFileRef: Option<FileRef>,
    pub Texture: Option<FileRef>,
    pub EffectTexture: Option<FileRef>,
    pub SpellBookPredictions: Option<SpellBookPredictions>,
    pub CombatLogTexts: Option<CombatLogTexts>,
    pub TrainedCost: i32,
    pub Level: i32,
    pub MagicSchool: String,
    pub damage: Option<SpellFormula>,
    pub duration: Option<SpellFormula>,
    pub sSpellCost: Resources,
    pub IsAimed: bool,
    pub IsAreaAttack: bool,
    pub CanSelectDead: bool,
    pub Target: String,
    pub Element: String,
    pub DamageIsElemental: Option<bool>,
    pub visuals: Option<Visuals>,
    pub PresetPrice: Option<i32>,
    pub AvailableForPresets: Option<bool>
}
