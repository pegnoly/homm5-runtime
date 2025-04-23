use sea_orm::ActiveValue::Set;

use crate::services::banks::models::bank_variant::BankDifficultyType;

pub struct CreateVariantPayload {
    pub bank_id: i32,
    pub chance: i32,
    pub difficulty: BankDifficultyType
}

#[derive(Debug, Default)]
pub struct UpdateBankPayload {
    pub id: i32,
    pub recharge_count: Option<i32>,
    pub recharge_timer: Option<i32>,
    pub luck_loss: Option<i32>,
    pub morale_loss: Option<i32>
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
    pub id: i32,
    pub chance: Option<i32>,
    pub difficulty: Option<BankDifficultyType>
}

impl UpdateBankVariantPayload {
    pub fn new(id: i32) -> Self {
        UpdateBankVariantPayload {
            id,
            ..Default::default()
        }
    }

    pub fn with_chance(mut self, chance: i32) -> Self {
        self.chance = Some(chance);
        self
    }

    pub fn with_difficulty(mut self, difficulty: BankDifficultyType) -> Self {
        self.difficulty = Some(difficulty);
        self
    }
}