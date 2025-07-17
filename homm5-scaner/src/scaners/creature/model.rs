use homm5_types::{common::FileRef, creature::{Abilities, Upgrades}};
use itertools::Itertools;
use sea_orm::{FromJsonQueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    core::ToLua,
    scaners::common::{MagicElement, Mastery, ResourcesModel, Town},
};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "creatures")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub game_id: String,
    pub attack: i32,
    pub defence: i32,
    pub min_damage: i32,
    pub max_damage: i32,
    pub speed: i32,
    pub initiative: i32,
    pub health: i32,
    pub is_flying: bool,
    pub known_spells: KnownSpellsModel,
    pub spell_points: i32,
    pub exp: i32,
    pub power: i32,
    pub tier: i32,
    pub magic_element: MagicElementModel,
    pub grow: i32,
    pub town: Town,
    pub cost: ResourcesModel,
    pub is_generatable: bool,
    pub shared: String,
    pub size: i32,
    pub range: i32,
    pub name_txt: String,
    pub name: String,
    pub desc_txt: String,
    pub desc: String,
    pub icon_xdb: String,
    pub base_creature: String,
    pub pair_creature: String,
    pub abilities: AbilitiesModel,
    pub upgrades: UpgradesModel,
    pub inner_name: Option<String>,
    #[serde(skip)]
    pub xdb: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct SpellWithMasteryModel {
    pub spell: String,
    pub mastery: Mastery,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct KnownSpellsModel {
    pub spells: Vec<SpellWithMasteryModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct MagicElementModel {
    pub first: MagicElement,
    pub second: MagicElement,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct AbilitiesModel {
    pub abilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct UpgradesModel {
    pub upgrades: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<homm5_types::creature::AdvMapCreatureShared> for Model {
    fn from(value: homm5_types::creature::AdvMapCreatureShared) -> Self {
        Model {
            id: Default::default(),
            game_id: Default::default(),
            attack: value.AttackSkill as i32,
            defence: value.DefenceSkill as i32,
            min_damage: value.MinDamage as i32,
            max_damage: value.MaxDamage as i32,
            speed: value.Speed as i32,
            health: value.Health as i32,
            initiative: value.Initiative as i32,
            is_flying: value.Flying,
            known_spells: if let Some(spells) = value.KnownSpells.spells {
                KnownSpellsModel {
                    spells: spells
                        .iter()
                        .map(|spell| SpellWithMasteryModel {
                            spell: spell.Spell.clone(),
                            mastery: Mastery::from_str(&spell.Mastery)
                                .unwrap_or(Mastery::MasteryNone),
                        })
                        .collect::<Vec<SpellWithMasteryModel>>(),
                }
            } else {
                KnownSpellsModel { spells: vec![] }
            },
            spell_points: value.SpellPoints as i32,
            exp: value.Exp as i32,
            power: value.Power as i32,
            tier: value.CreatureTier as i32,
            magic_element: MagicElementModel {
                first: MagicElement::from_str(&value.MagicElement.First)
                    .unwrap_or(MagicElement::ElementNone),
                second: MagicElement::from_str(&value.MagicElement.Second)
                    .unwrap_or(MagicElement::ElementNone),
            },
            town: Town::from_str(&value.CreatureTown).unwrap_or(Town::TownNoType),
            grow: value.WeeklyGrowth as i32,
            is_generatable: value.SubjectOfRandomGeneration,
            cost: ResourcesModel {
                wood: value.Cost.Wood as i32,
                ore: value.Cost.Ore as i32,
                mercury: value.Cost.Mercury as i32,
                sulfur: value.Cost.Sulfur as i32,
                crystal: value.Cost.Crystal as i32,
                gem: value.Cost.Gem as i32,
                gold: value.Cost.Gold as i32,
            },
            shared: if let Some(shared) = value.MonsterShared {
                shared.href.unwrap_or(String::new())
            } else {
                String::new()
            },
            size: value.CombatSize as i32,
            range: value.Range as i32,
            name_txt: if let Some(ref visual) = value.VisualExplained {
                visual
                    .CreatureNameFileRef
                    .as_ref()
                    .unwrap_or(&FileRef {
                        href: Some(String::new()),
                    })
                    .href
                    .clone()
                    .unwrap_or(String::new())
            } else {
                String::new()
            },
            name: Default::default(),
            desc_txt: if let Some(ref visual) = value.VisualExplained {
                visual
                    .DescriptionFileRef
                    .as_ref()
                    .unwrap_or(&FileRef {
                        href: Some(String::new()),
                    })
                    .href
                    .clone()
                    .unwrap_or(String::new())
            } else {
                String::new()
            },
            desc: Default::default(),
            icon_xdb: if let Some(ref visual) = value.VisualExplained {
                visual
                    .Icon128
                    .as_ref()
                    .unwrap_or(&FileRef {
                        href: Some(String::new()),
                    })
                    .href
                    .clone()
                    .unwrap_or(String::new())
            } else {
                String::new()
            },
            abilities: AbilitiesModel {
                abilities: if let Some(abilities) = value.Abilities.unwrap_or(Abilities { Abilities: None }).Abilities {
                    abilities
                } else {
                    vec![]
                },
            },
            upgrades: UpgradesModel {
                upgrades: if let Some(upgrades) = value.Upgrades.unwrap_or(Upgrades { upgrages: None}).upgrages {
                    upgrades
                } else {
                    vec![]
                },
            },
            base_creature: value.BaseCreature.unwrap_or("CREATURE_UNKNOWN".to_string()),
            pair_creature: value.PairCreature,
            inner_name: value.InnerName,
            xdb: None
        }
    }
}

impl ToLua for Model {
    fn to_lua_string(&self) -> String {
        let is_generatable = if self.is_generatable == true {
            "1"
        } else {
            "nil"
        };
        let is_flying = if self.is_flying == true { "1" } else { "nil" };
        let is_upgrade =
            if self.base_creature == "CREATURE_UNKNOWN" && self.upgrades.upgrades.len() > 0 {
                "nil"
            } else {
                "1"
            };
        let abilities_string = self.abilities.abilities.iter().join(", ");
        let spells_string = self
            .known_spells
            .spells
            .iter()
            .map(|s| format!("[{}] = {}", &s.spell, &s.mastery))
            .join(", ");
        format!(
            "\t[{}] = {{
        is_generatable = {},
        is_upgrade = {},
        attack = {},
        defence = {},
        dmg_min = {},
        dmg_max = {},
        speed = {},
        ini = {},
        health = {},
        sp = {},
        size = {},
        exp = {},
        power = {},
        town = {},
        first_element = {},
        second_element = {},
        grow = {},
        tier = {},
        cost = {},
        range = {},
        name = \"{}\",
        desc = \"{}\",
        icon = \"{}\",
        is_flying = {},
        abilities = {{{}}},
        known_spells = {{{}}}
    }},\n",
            self.id,
            is_generatable,
            is_upgrade,
            self.attack,
            self.defence,
            self.min_damage,
            self.max_damage,
            self.speed,
            self.initiative,
            self.health,
            self.spell_points,
            self.size,
            self.exp,
            self.power,
            self.town,
            self.magic_element.first,
            self.magic_element.second,
            self.grow,
            self.tier,
            self.cost.gold,
            self.range,
            self.name_txt,
            self.desc_txt,
            self.icon_xdb,
            is_flying,
            abilities_string,
            spells_string
        )
    }
}
