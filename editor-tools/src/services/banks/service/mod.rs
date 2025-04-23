use std::path::PathBuf;
use itertools::Itertools;
use payloads::{CreateVariantPayload, UpdateBankPayload, UpdateBankVariantPayload};
use sea_orm::{sqlx::{types::Json, SqlitePool}, ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityOrSelect, EntityTrait, FromQueryResult, InsertResult, IntoActiveModel, ModelTrait, QueryFilter, QuerySelect, QueryTrait, Related, SqlxSqlitePoolConnection};
use crate::error::EditorToolsError;
use super::models::{bank, bank_creature_entry::{self, BankCreatureSlotType, CreatureSlotData}, bank_variant};

pub mod payloads;

pub struct BanksService {
    db: DatabaseConnection
}

impl BanksService {
    pub fn new(pool: SqlitePool) -> Self {
        BanksService { db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)) }
    }

    pub async fn get_banks(&self) -> Result<Vec<super::models::bank::Model>, EditorToolsError> {
        Ok(bank::Entity::find().all(&self.db).await?)
    }

    pub async fn get_bank(&self, id: i32) -> Result<Option<super::models::bank::Model>, EditorToolsError> {
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

    pub async fn get_variant(&self, id: i32) -> Result<Option<bank_variant::Model>, EditorToolsError> {
        Ok(bank_variant::Entity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn get_variants(&self, bank_id: i32) -> Result<Vec<bank_variant::Model>, EditorToolsError> {
        Ok(bank_variant::Entity::find().filter(bank_variant::Column::BankId.eq(bank_id)).all(&self.db).await?)
    }

    pub async fn create_variant(&self, payload: CreateVariantPayload) -> Result<bank_variant::Model, EditorToolsError> {
        let model_to_insert = bank_variant::ActiveModel {
            bank_id: Set(payload.bank_id),
            chance: Set(payload.chance),
            difficulty: Set(payload.difficulty),
            ..Default::default()
        };
        Ok(model_to_insert.insert(&self.db).await?)
    }

    pub async fn update_variant(&self, payload: UpdateBankVariantPayload) -> Result<(), EditorToolsError> {
        if let Some(current_variant) = bank_variant::Entity::find_by_id(payload.id).one(&self.db).await? {
            let mut model_to_update = current_variant.into_active_model();
            if let Some(chance) = payload.chance {
                model_to_update.chance = Set(chance);
            }
            if let Some(difficulty) = payload.difficulty {
                model_to_update.difficulty = Set(difficulty);
            }
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn delete_variant(&self, id: i32) -> Result<(), EditorToolsError> {
        if let Some(model_to_delete) =  bank_variant::Entity::find_by_id(id).one(&self.db).await? {
            model_to_delete.delete(&self.db).await?;
        }
        Ok(())
    }

    pub async fn create_creature_entry(&self, variant_id: i32, slot_type: BankCreatureSlotType, data: CreatureSlotData) -> Result<i32, EditorToolsError> {
        let model_to_insert = bank_creature_entry::ActiveModel {
            variant_id: Set(variant_id),
            _type: Set(slot_type),
            data: Set(data),
            ..Default::default()
        };
        let inserted_model = model_to_insert.insert(&self.db).await?;
        Ok(inserted_model.id)
    }

    pub async fn load_creature_entries(&self, variant_id: i32) -> Result<Vec<i32>, EditorToolsError> {
        let statement = bank_creature_entry::Entity::find()
            .select_only()
            .column(bank_creature_entry::Column::Id)
            .filter(bank_creature_entry::Column::VariantId.eq(variant_id))
            .build(sea_orm::DatabaseBackend::Sqlite);

        let query_result = bank_creature_entry::CreatureEntryId::find_by_statement(statement).all(&self.db).await?;
        Ok(query_result.into_iter().map(|e| {
            e.id
        }).collect_vec())
    }

    pub async fn load_creature_entry(&self, id: i32) -> Result<Option<bank_creature_entry::Model>, EditorToolsError> {
        Ok(bank_creature_entry::Entity::find_by_id(id).one(&self.db).await?)
    }
}