use std::collections::HashMap;
use sea_orm::{sqlx::SqlitePool, ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel, SqlxSqlitePoolConnection};
use crate::error::EditorToolsError;
use super::{models::{self, generatable_hero::{self, OptionalArtifactsModel}}, payloads::{AddOptionalArtifactPayload, InitGeneratableHeroPayload, RemoveOptionalArtifactPayload, UpdateArtifactCostTypePayload}};

pub struct HeroGeneratorRepo {
    db: DatabaseConnection
}

impl HeroGeneratorRepo {
    pub fn new(pool: SqlitePool) -> Self {
        HeroGeneratorRepo { db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)) }
    }

    pub async fn init_new_generatable_hero(&self, payload: InitGeneratableHeroPayload) -> Result<(), EditorToolsError> {
        let model_to_insert = generatable_hero::ActiveModel {
            id: Default::default(),
            table_name: Set(payload.lua_table_name),
            path_to_generate: Set(payload.path_to_generate),
            optional_artifacts: Set(OptionalArtifactsModel { artifacts: HashMap::new(), cost_type: generatable_hero::ArtifactsGenerationCostType::NotSelected})
        };
        model_to_insert.insert(&self.db).await?;
        Ok(())
    }

    pub async fn update_optional_artifacts_cost_type(&self, payload: UpdateArtifactCostTypePayload) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = generatable_hero::Entity::find_by_id(payload.id).one(&self.db).await? {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut optional_artifacts_to_update = existing_model.optional_artifacts.clone();
            optional_artifacts_to_update.cost_type = payload.new_type;
            model_to_update.optional_artifacts = Set(optional_artifacts_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn add_optional_artifact_id(&self, payload: AddOptionalArtifactPayload) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = generatable_hero::Entity::find_by_id(payload.hero_id).one(&self.db).await? {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut optional_artifacts_to_update = existing_model.optional_artifacts.clone();
            if let Some(ids) = optional_artifacts_to_update.artifacts.get_mut(&payload.slot) {
                ids.push(payload.artifact_id);
            } else {
                optional_artifacts_to_update.artifacts.insert(payload.slot, vec![payload.artifact_id]);
            }
            model_to_update.optional_artifacts = Set(optional_artifacts_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn remove_optional_artifact_id(&self, payload: RemoveOptionalArtifactPayload) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = generatable_hero::Entity::find_by_id(payload.hero_id).one(&self.db).await? {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut optional_artifacts_to_update = existing_model.optional_artifacts.clone();
            if let Some(ids) = optional_artifacts_to_update.artifacts.get_mut(&payload.slot) {
                ids.retain(|id| *id != payload.artifact_id);
            }
            model_to_update.optional_artifacts = Set(optional_artifacts_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }
}