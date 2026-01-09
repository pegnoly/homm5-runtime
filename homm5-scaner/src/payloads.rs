use crate::prelude::{MagicElementModel, ResourcesModel, Town};

#[derive(Default)]
pub struct UpdateCreaturePayload {
    pub id: i32,
    pub attack: Option<i32>,
    pub defence: Option<i32>,
    pub min_damage: Option<i32>,
    pub max_damage: Option<i32>,
    pub speed: Option<i32>,
    pub initiative: Option<i32>,
    pub health: Option<i32>,
    pub spell_points: Option<i32>,
    pub exp: Option<i32>,
    pub power: Option<i32>,
    pub tier: Option<i32>,
    pub grow: Option<i32>,
    pub town: Option<Town>,
    pub size: Option<i32>,
    pub range: Option<i32>,
    pub shots: Option<i32>,
    pub pair_creature: Option<String>,
    pub base_creature: Option<String>,
    pub upgrades: Option<Vec<String>>,
    pub abilities: Option<Vec<String>>,
    pub magic_element: Option<MagicElementModel>,
    pub cost: Option<ResourcesModel>,
    pub is_generatable: Option<bool>,
    pub is_flying: Option<bool>,
    pub is_upgrade: Option<bool>,
    pub desc: Option<String>,
    pub name: Option<String>
}

impl UpdateCreaturePayload {
    pub fn new(id: i32) -> Self {
        UpdateCreaturePayload {
            id,
            ..Default::default()
        }
    }

    pub fn with_attack(mut self, attack: i32) -> Self {
        self.attack = Some(attack);
        self
    }

    pub fn with_defence(mut self, defence: i32) -> Self {
        self.defence = Some(defence);
        self
    }

    pub fn with_min_damage(mut self, min_damage: i32) -> Self {
        self.min_damage = Some(min_damage);
        self
    }

    pub fn with_max_damage(mut self, max_damage: i32) -> Self {
        self.max_damage = Some(max_damage);
        self
    }

    pub fn with_speed(mut self, speed: i32) -> Self {
        self.speed = Some(speed);
        self
    }

    pub fn with_initiative(mut self, initiative: i32) -> Self {
        self.initiative = Some(initiative);
        self
    }

    pub fn with_health(mut self, health: i32) -> Self {
        self.health = Some(health);
        self
    }

    pub fn with_spell_points(mut self, spell_points: i32) -> Self {
        self.spell_points = Some(spell_points);
        self
    }

    pub fn with_exp(mut self, exp: i32) -> Self {
        self.exp = Some(exp);
        self
    }

    pub fn with_power(mut self, power: i32) -> Self {
        self.power = Some(power);
        self
    }

    pub fn with_tier(mut self, tier: i32) -> Self {
        self.tier = Some(tier);
        self
    }

    pub fn with_grow(mut self, grow: i32) -> Self {
        self.grow = Some(grow);
        self
    }

    pub fn with_town(mut self, town: Town) -> Self {
        self.town = Some(town);
        self
    }

    pub fn with_size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_range(mut self, range: i32) -> Self {
        self.range = Some(range);
        self
    }

    pub fn with_shots(mut self, shots: i32) -> Self {
        self.shots = Some(shots);
        self
    }

    pub fn with_base_creature(mut self, base_creature: String) -> Self {
        self.base_creature = Some(base_creature);
        self
    }

    pub fn with_pair_creature(mut self, pair_creature: String) -> Self {
        self.pair_creature = Some(pair_creature);
        self
    }

    pub fn with_upgrades(mut self, upgrades: Vec<String>) -> Self {
        self.upgrades = Some(upgrades);
        self
    }

    pub fn with_abilities(mut self, abilities: Vec<String>) -> Self {
        self.abilities = Some(abilities);
        self
    }

    pub fn with_magic_element(mut self, element: MagicElementModel) -> Self {
        self.magic_element = Some(element);
        self
    }

    pub fn with_cost(mut self, cost: ResourcesModel) -> Self {
        self.cost = Some(cost);
        self
    }

    pub fn with_generatable(mut self, is_generatable: bool) -> Self {
        self.is_generatable = Some(is_generatable);
        self
    }

    pub fn with_flying(mut self, is_flying: bool) -> Self {
        self.is_flying = Some(is_flying);
        self
    }

    pub fn with_upgrade(mut self, is_upgrade: bool) -> Self {
        self.is_upgrade = Some(is_upgrade);
        self
    }

    pub fn with_desc(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}