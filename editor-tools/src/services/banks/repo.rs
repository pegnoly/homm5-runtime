use super::models::{
    self, bank,
    bank_creature_entry::{self, BankCreatureSlotType, CreatureSlotData},
    bank_difficulty, bank_variant,
};
use super::payloads::*;
use crate::{
    error::EditorToolsError,
    prelude::{BankDifficultyDBModel, BankDifficultyType},
};
use itertools::Itertools;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    FromQueryResult, IntoActiveModel, ModelTrait, QueryFilter, QuerySelect, QueryTrait,
    SqlxSqlitePoolConnection, prelude::Uuid, sqlx::SqlitePool,
};
use std::path::PathBuf;

pub struct BanksGeneratorRepo {
    db: DatabaseConnection,
    pub path: PathBuf,
}

impl BanksGeneratorRepo {
    pub fn new(pool: SqlitePool, path: PathBuf) -> Self {
        BanksGeneratorRepo {
            db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)),
            path,
        }
    }

    pub async fn get_banks(&self) -> Result<Vec<models::bank::Model>, EditorToolsError> {
        let models = bank::Entity::find().all(&self.db).await?;
        println!("Models: {:#?}", &models);
        Ok(models)
    }

    pub async fn get_bank(
        &self,
        id: i32,
    ) -> Result<Option<super::models::bank::Model>, EditorToolsError> {
        Ok(bank::Entity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn update_bank(&self, payload: UpdateBankPayload) -> Result<(), EditorToolsError> {
        if let Some(current_model) = bank::Entity::find_by_id(payload.id).one(&self.db).await? {
            let mut model_to_update: bank::ActiveModel = current_model.into();
            if let Some(count) = payload.recharge_count {
                model_to_update.recharge_count = Set(count);
            }
            if let Some(timer) = payload.recharge_timer {
                model_to_update.recharge_timer = Set(timer);
            }
            if let Some(loss) = payload.luck_loss {
                model_to_update.luck_loss = Set(loss);
            }
            if let Some(loss) = payload.morale_loss {
                model_to_update.morale_loss = Set(loss);
            }
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn load_difficulty(
        &self,
        bank_id: i32,
        difficulty: BankDifficultyType,
    ) -> Result<Option<BankDifficultyDBModel>, EditorToolsError> {
        Ok(bank_difficulty::Entity::find()
            .filter(bank_difficulty::Column::BankId.eq(bank_id))
            .filter(bank_difficulty::Column::DifficultyType.eq(difficulty))
            .one(&self.db)
            .await?)
    }

    pub async fn update_difficulty(&self, id: i32, chance: i32) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = bank_difficulty::Entity::find_by_id(id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.chance = Set(chance);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn get_variant(
        &self,
        id: Uuid,
    ) -> Result<Option<bank_variant::Model>, EditorToolsError> {
        Ok(bank_variant::Entity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn get_variants(
        &self,
        bank_id: i32,
        difficulty: BankDifficultyType,
    ) -> Result<Vec<bank_variant::Model>, EditorToolsError> {
        Ok(bank_variant::Entity::find()
            .filter(bank_variant::Column::BankId.eq(bank_id))
            .filter(bank_variant::Column::Difficulty.eq(difficulty))
            .all(&self.db)
            .await?)
    }

    pub async fn create_variant(
        &self,
        payload: CreateVariantPayload,
    ) -> Result<bank_variant::Model, EditorToolsError> {
        let model_to_insert = bank_variant::ActiveModel {
            bank_id: Set(payload.bank_id),
            label: Set(payload.label),
            difficulty: Set(payload.difficulty),
            id: Set(Uuid::new_v4()),
        };
        Ok(model_to_insert.insert(&self.db).await?)
    }

    pub async fn update_variant(
        &self,
        payload: UpdateBankVariantPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(current_variant) = bank_variant::Entity::find_by_id(payload.id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = current_variant.into_active_model();
            if let Some(label) = payload.label {
                model_to_update.label = Set(label);
            }
            if let Some(difficulty) = payload.difficulty {
                model_to_update.difficulty = Set(difficulty);
            }
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn delete_variant(&self, id: Uuid) -> Result<(), EditorToolsError> {
        if let Some(model_to_delete) = bank_variant::Entity::find_by_id(id).one(&self.db).await? {
            model_to_delete.delete(&self.db).await?;
        }
        Ok(())
    }

    pub async fn create_creature_entry(
        &self,
        variant_id: i32,
        slot_type: BankCreatureSlotType,
        data: CreatureSlotData,
    ) -> Result<i32, EditorToolsError> {
        let model_to_insert = bank_creature_entry::ActiveModel {
            variant_id: Set(variant_id),
            _type: Set(slot_type),
            data: Set(data),
            ..Default::default()
        };
        let inserted_model = model_to_insert.insert(&self.db).await?;
        Ok(inserted_model.id)
    }

    pub async fn load_creature_entries(
        &self,
        variant_id: i32,
    ) -> Result<Vec<i32>, EditorToolsError> {
        let statement = bank_creature_entry::Entity::find()
            .select_only()
            .column(bank_creature_entry::Column::Id)
            .filter(bank_creature_entry::Column::VariantId.eq(variant_id))
            .build(sea_orm::DatabaseBackend::Sqlite);

        let query_result = bank_creature_entry::CreatureEntryId::find_by_statement(statement)
            .all(&self.db)
            .await?;
        Ok(query_result.into_iter().map(|e| e.id).collect_vec())
    }

    pub async fn load_full_creature_entries(
        &self,
        variant_id: i32,
    ) -> Result<Vec<bank_creature_entry::Model>, EditorToolsError> {
        Ok(bank_creature_entry::Entity::find()
            .filter(bank_creature_entry::Column::VariantId.eq(variant_id))
            .all(&self.db)
            .await?)
    }

    pub async fn load_creature_entry(
        &self,
        id: i32,
    ) -> Result<Option<bank_creature_entry::Model>, EditorToolsError> {
        Ok(bank_creature_entry::Entity::find_by_id(id)
            .one(&self.db)
            .await?)
    }

    pub async fn update_creature_entry(
        &self,
        payload: UpdateCreatureEntryPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(current_model) = bank_creature_entry::Entity::find_by_id(payload.id)
            .one(&self.db)
            .await?
        {
            let mut entry_data = current_model.data.clone();
            let mut model_to_update = current_model.into_active_model();
            if let Some(power) = payload.base_power {
                entry_data.base_power = Some(power);
            }
            if let Some(grow) = payload.power_grow {
                entry_data.power_grow = Some(grow);
            }
            if let Some(town) = payload.creature_town {
                entry_data.creature_town = Some(town);
            }
            if let Some(tier) = payload.creature_tier {
                entry_data.creature_tier = Some(tier);
            }
            if let Some(creature_id) = payload.creature_id {
                entry_data.creature_id = Some(creature_id);
            }
            if let Some(count) = payload.creature_count {
                entry_data.creature_count = Some(count);
            }
            model_to_update.data = Set(entry_data);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }
}
