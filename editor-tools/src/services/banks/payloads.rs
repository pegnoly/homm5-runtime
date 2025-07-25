use crate::services::banks::models::bank_difficulty::BankDifficultyType;
use homm5_scaner::prelude::Town;
use sea_orm::prelude::Uuid;

pub struct CreateVariantPayload {
    pub bank_id: i32,
    pub label: String,
    pub difficulty: BankDifficultyType,
}

#[derive(Debug, Default)]
pub struct UpdateBankPayload {
    pub id: i32,
    pub recharge_count: Option<i32>,
    pub recharge_timer: Option<i32>,
    pub luck_loss: Option<i32>,
    pub morale_loss: Option<i32>,
}

impl UpdateBankPayload {
    pub fn new(bank_id: i32) -> Self {
        UpdateBankPayload {
            id: bank_id,
            ..Default::default()
        }
    }

    pub fn with_recharge_count(mut self, count: i32) -> Self {
        self.recharge_count = Some(count);
        self
    }

    pub fn with_recharge_timer(mut self, timer: i32) -> Self {
        self.recharge_timer = Some(timer);
        self
    }

    pub fn with_luck_loss(mut self, loss: i32) -> Self {
        self.luck_loss = Some(loss);
        self
    }

    pub fn with_morale_loss(mut self, loss: i32) -> Self {
        self.morale_loss = Some(loss);
        self
    }
}

#[derive(Debug, Default)]
pub struct UpdateBankVariantPayload {
    pub id: Uuid,
    pub label: Option<String>,
    pub difficulty: Option<BankDifficultyType>,
}

impl UpdateBankVariantPayload {
    pub fn new(id: Uuid) -> Self {
        UpdateBankVariantPayload {
            id,
            ..Default::default()
        }
    }

    pub fn with_difficulty(mut self, difficulty: BankDifficultyType) -> Self {
        self.difficulty = Some(difficulty);
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
}

#[derive(Debug, Default)]
pub struct UpdateCreatureEntryPayload {
    pub id: i32,
    pub base_power: Option<i32>,
    pub power_grow: Option<i32>,
    pub creature_town: Option<Town>,
    pub creature_tier: Option<i32>,
    pub creature_id: Option<i32>,
    pub creature_count: Option<i32>,
}

impl UpdateCreatureEntryPayload {
    pub fn new(id: i32) -> Self {
        UpdateCreatureEntryPayload {
            id,
            ..Default::default()
        }
    }

    pub fn with_base_power(mut self, power: i32) -> Self {
        self.base_power = Some(power);
        self
    }

    pub fn with_power_grow(mut self, grow: i32) -> Self {
        self.power_grow = Some(grow);
        self
    }

    pub fn with_town(mut self, town: Town) -> Self {
        self.creature_town = Some(town);
        self
    }

    pub fn with_tier(mut self, tier: i32) -> Self {
        self.creature_tier = Some(tier);
        self
    }

    pub fn with_creature_id(mut self, id: i32) -> Self {
        self.creature_id = Some(id);
        self
    }

    pub fn with_count(mut self, count: i32) -> Self {
        self.creature_count = Some(count);
        self
    }
}
