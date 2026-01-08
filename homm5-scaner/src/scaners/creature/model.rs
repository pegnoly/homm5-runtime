use homm5_types::{
    common::FileRef,
    creature::{Abilities, AdvMapCreatureShared, KnownSpells, MagicElement, Resources, Spell, Upgrades},
};
use itertools::Itertools;
use sea_orm::{FromJsonQueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    core::ToLua,
    scaners::common::{MagicElement as DBMagicElement, Mastery, ResourcesModel, Town},
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
    pub xdb: Option<String>,
    pub xdb_path: String,
    pub is_upgrade: bool,
    pub shots: i32,
    pub unused_data: UnusedCreatureDataModel
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
    pub first: DBMagicElement,
    pub second: DBMagicElement
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct AbilitiesModel {
    pub abilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct UpgradesModel {
    pub upgrades: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct UnusedCreatureDataModel {
    pub spell_points_1: i32,
    pub spell_points_2: i32,
    pub time_to_command: i32,
    pub pattern_attack: Option<String>,
    pub flyby_sequence: Option<String>,
    pub visual_path: Option<String>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<homm5_types::creature::AdvMapCreatureShared> for Model {
    fn from(value: homm5_types::creature::AdvMapCreatureShared) -> Self {
        Model {
            id: Default::default(),
            game_id: Default::default(),
            attack: value.AttackSkill,
            defence: value.DefenceSkill,
            min_damage: value.MinDamage,
            max_damage: value.MaxDamage,
            speed: value.Speed,
            health: value.Health,
            initiative: value.Initiative,
            is_flying: value.Flying,
            known_spells: if let Some(known_spells) = value.KnownSpells {
                if let Some(spells) = known_spells.spells {
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
                }
            } else {
                KnownSpellsModel { spells: vec![] }
            },
            spell_points: value.SpellPoints,
            exp: value.Exp,
            power: value.Power,
            tier: value.CreatureTier,
            magic_element: MagicElementModel {
                first: DBMagicElement::from_str(&value.MagicElement.First)
                    .unwrap_or(DBMagicElement::ElementNone),
                second: DBMagicElement::from_str(&value.MagicElement.Second)
                    .unwrap_or(DBMagicElement::ElementNone),
            },
            town: Town::from_str(&value.CreatureTown).unwrap_or(Town::TownNoType),
            grow: value.WeeklyGrowth,
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
            size: value.CombatSize,
            range: value.Range,
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
                abilities: value
                    .Abilities
                    .unwrap_or(Abilities { Abilities: None })
                    .Abilities
                    .unwrap_or_default(),
            },
            upgrades: UpgradesModel {
                upgrades: value
                    .Upgrades
                    .unwrap_or(Upgrades { upgrages: None })
                    .upgrages
                    .unwrap_or_default(),
            },
            base_creature: value.BaseCreature.unwrap_or("CREATURE_UNKNOWN".to_string()),
            pair_creature: value.PairCreature,
            inner_name: value.InnerName,
            xdb: None,
            xdb_path: String::new(),
            is_upgrade: value.Upgrade,
            shots: value.Shots,
            unused_data: UnusedCreatureDataModel { 
                spell_points_1: value.SpellPoints1, 
                spell_points_2: value.SpellPoints2, 
                time_to_command: value.TimeToCommand, 
                pattern_attack: Some(serde_json::to_string_pretty(&value.PatternAttack).unwrap()),
                flyby_sequence: Some(serde_json::to_string_pretty(&value.flybySequence).unwrap()),
                visual_path: if let Some(visual) = value.Visual {
                    visual.href
                } else {
                    None
                }
            }
        }
    }
}

impl From<Model> for AdvMapCreatureShared  {
    fn from(value: Model) -> Self {
        AdvMapCreatureShared { 
            AttackSkill: value.attack, 
            DefenceSkill: value.defence, 
            MinDamage: value.min_damage, 
            MaxDamage: value.max_damage, 
            Speed: value.speed, 
            Initiative: value.initiative, 
            Flying: value.is_flying, 
            Health: value.health, 
            KnownSpells: if value.known_spells.spells.is_empty() {
                None
            } else {
                Some(KnownSpells { spells: Some(
                    Vec::from_iter(value.known_spells.spells.iter().map(|sp| {
                        Spell {
                            Spell: sp.spell.clone(),
                            Mastery: sp.mastery.to_string()
                        }
                    }))
                )})
            },
            SpellPoints: value.spell_points, 
            Exp: value.exp, 
            Power: value.power, 
            CreatureTier: value.tier, 
            Upgrade: value.is_upgrade, 
            PairCreature: value.pair_creature, 
            CreatureTown: value.town.to_string(), 
            MagicElement: MagicElement {
                First: value.magic_element.first.to_string(),
                Second: value.magic_element.second.to_string()
            }, 
            WeeklyGrowth: value.grow, 
            Cost: Resources {
                Gold: value.cost.gold as u32,
                Wood: value.cost.wood as u16,
                Ore: value.cost.ore as u16,
                Mercury: value.cost.mercury as u16,
                Crystal: value.cost.crystal as u16,
                Sulfur: value.cost.sulfur as u16,
                Gem: value.cost.gem as u16,
            }, 
            SubjectOfRandomGeneration: value.is_generatable, 
            MonsterShared: Some(FileRef { href: Some(value.shared)}), 
            CombatSize: value.size, 
            Visual: Some(FileRef { href: value.unused_data.visual_path }), 
            Range: value.range, 
            BaseCreature: Some(value.base_creature), 
            Upgrades: if value.upgrades.upgrades.is_empty() {
                Some(Upgrades { upgrages: None })
            } else {
                Some(Upgrades { upgrages: Some(value.upgrades.upgrades) })
            },  
            Abilities: if value.abilities.abilities.is_empty() {
                Some(Abilities { Abilities: None })
            } else {
                Some(Abilities { Abilities: Some(value.abilities.abilities) })
            }, 
            VisualExplained: None, 
            InnerName: value.inner_name,
            Shots: value.shots,
            SpellPoints1: value.unused_data.spell_points_1,
            SpellPoints2: value.unused_data.spell_points_2,
            PatternAttack: value.unused_data.pattern_attack.map(|data| serde_json::from_str(&data).unwrap()),
            TimeToCommand: value.unused_data.time_to_command,
            flybySequence: value.unused_data.flyby_sequence.map(|data| serde_json::from_str(&data).unwrap())
        }
    }
}

impl ToLua for Model {
    fn to_lua_string(&self) -> String {
        let is_generatable = if self.is_generatable { "1" } else { "nil" };
        let is_flying = if self.is_flying { "1" } else { "nil" };
        let is_upgrade =
            if self.base_creature == "CREATURE_UNKNOWN" && !self.upgrades.upgrades.is_empty() {
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
