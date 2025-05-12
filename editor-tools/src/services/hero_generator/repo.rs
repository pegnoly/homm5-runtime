use super::{
    models::{
        artifacts::{self, OptionalArtifacts, RequiredArtifacts}, asset, common::{AssetGenerationType, DifficultyMappedValue}
    },
    payloads::{
        AddOptionalArtifactPayload, InitGeneratableHeroPayload, RemoveOptionalArtifactPayload,
    },
    prelude::{InitAssetArtifactsDataPayload, UpdateArtifactsGenerationPowerPayload, UpdateArtifactsGenerationTypePayload},
};
use crate::error::EditorToolsError;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel,
    SqlxSqlitePoolConnection, sqlx::SqlitePool,
};

pub struct HeroGeneratorRepo {
    db: DatabaseConnection,
}

impl HeroGeneratorRepo {
    pub fn new(pool: SqlitePool) -> Self {
        HeroGeneratorRepo {
            db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)),
        }
    }

    pub async fn get_all_hero_assets(
        &self
    ) -> Result<Vec<asset::Model>, EditorToolsError> {
        Ok(asset::Entity::find().all(&self.db).await?)
    }

    pub async fn get_hero_asset(
        &self,
        id: i32
    ) -> Result<Option<asset::Model>, EditorToolsError> {
        Ok(asset::Entity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn init_new_generatable_hero(
        &self,
        payload: InitGeneratableHeroPayload,
    ) -> Result<asset::Model, EditorToolsError> {
        let model_to_insert = asset::ActiveModel {
            name: Set(payload.name),
            table_name: Set(payload.lua_table_name),
            path_to_generate: Set(payload.path_to_generate),
            ..Default::default()
        };
        let model = model_to_insert.insert(&self.db).await?;
        Ok(model)
    }

    pub async fn add_artifacts_model(
        &self,
        payload: InitAssetArtifactsDataPayload,
    ) -> Result<artifacts::Model, EditorToolsError> {
        let model_to_insert = artifacts::ActiveModel {
            id: Default::default(),
            asset_id: Set(payload.asset_id),
            generation_type: Set(payload.generation_type.clone()),
            base_powers: Set(DifficultyMappedValue::default()),
            powers_grow: match payload.generation_type {
                AssetGenerationType::Static => Set(None),
                AssetGenerationType::Dynamic => Set(Some(DifficultyMappedValue::default())),
            },
            required: Set(RequiredArtifacts::default()),
            optional: Set(OptionalArtifacts::default()),
        };
        Ok(model_to_insert.insert(&self.db).await?)
    }

    pub async fn update_artifacts_generation_type(
        &self,
        payload: UpdateArtifactsGenerationTypePayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.id).one(&self.db).await? {
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.generation_type = Set(payload.new_type);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_artifacts_base_generation_power(
        &self,
        payload: UpdateArtifactsGenerationPowerPayload
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.id).one(&self.db).await? {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut base_powers_to_update = existing_model.base_powers.clone();
            if let Some(power) = base_powers_to_update.data.get_mut(&payload.difficulty) {
                *power = payload.value;
            }
            model_to_update.base_powers = Set(base_powers_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_artifacts_grow_power(
        &self,
        payload: UpdateArtifactsGenerationPowerPayload
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.id).one(&self.db).await? {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut powers_grow_to_update = existing_model.powers_grow.unwrap().clone();
            if let Some(power) = powers_grow_to_update.data.get_mut(&payload.difficulty) {
                *power = payload.value;
            }
            model_to_update.powers_grow = Set(Some(powers_grow_to_update));
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }


    pub async fn add_optional_artifact_id(
        &self,
        payload: AddOptionalArtifactPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.hero_id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut optional_artifacts_to_update = existing_model.optional.clone();
            if let Some(ids) = optional_artifacts_to_update
                .values
                .get_mut(&payload.slot)
            {
                ids.push(payload.artifact_id);
            } else {
                optional_artifacts_to_update
                    .values
                    .insert(payload.slot, vec![payload.artifact_id]);
            }
            model_to_update.optional = Set(optional_artifacts_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn remove_optional_artifact_id(
        &self,
        payload: RemoveOptionalArtifactPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.hero_id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut optional_artifacts_to_update = existing_model.optional.clone();
            if let Some(ids) = optional_artifacts_to_update
                .values
                .get_mut(&payload.slot)
            {
                ids.retain(|id| *id != payload.artifact_id);
            }
            model_to_update.optional = Set(optional_artifacts_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }
}
