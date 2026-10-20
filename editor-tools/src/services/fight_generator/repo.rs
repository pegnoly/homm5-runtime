use super::{
    models::{
        army_slot::{self, ArmySlotId},
        artifacts::{self, OptionalArtifacts, RequiredArtifacts},
        asset,
        common::DifficultyMappedValue,
    },
    payloads::{AddOptionalArtifactPayload, RemoveOptionalArtifactPayload},
    prelude::{
        AddRequiredArtifactPayload, AddStackPayload,
        InitAssetArtifactsDataPayload, RemoveRequiredArtifactPayload,
        UpdateArtifactsGenerationTypePayload, UpdateDifficultyBasedPowerPayload,
        UpdateGenerationRulesPayload
    },
};
use crate::{
    error::EditorToolsError,
    prelude::{
        AssetArmySlotModel, InitFightAssetPayload, UpdateFightAssetPayload, UpdateStackBaseDataPayload, UpdateStackTiersPayload, UpdateStackTownsPayload
    },
    services::fight_generator::models::army_slot::{CreatureIds, CreatureTiers, CreatureTowns},
};
use itertools::Itertools;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait, QueryFilter, SqlxSqlitePoolConnection, sqlx::SqlitePool, Condition};
use sea_orm::prelude::Expr;
use crate::prelude::{GetAssetsRequest, UpdateStackSpecificCreaturesPayload, UpdateStatRulesPayload};
use crate::services::fight_generator::models::army_slot::{StackGenerationRules, StackStatsRules};

pub struct FightGeneratorRepo {
    db: DatabaseConnection,
}

impl FightGeneratorRepo {
    pub fn new(pool: SqlitePool) -> Self {
        FightGeneratorRepo {
            db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)),
        }
    }

    pub async fn get_assets(
        &self,
        request: GetAssetsRequest
    ) -> Result<Vec<asset::Model>, EditorToolsError> {
        let condition = Condition::all()
            .add_option(match request {
                GetAssetsRequest::All => None,
                GetAssetsRequest::OfMission(m) => Some(Expr::col(asset::Column::MissionId).eq(m)),
                GetAssetsRequest::OfIds(ids) => Some(Expr::col(asset::Column::Id).is_in(ids))
            });

        Ok(asset::Entity::find().filter(condition).all(&self.db).await?)
    }

    pub async fn get_asset(&self, id: i32) -> Result<Option<asset::Model>, EditorToolsError> {
        Ok(asset::Entity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn init_new_asset(
        &self,
        payload: InitFightAssetPayload,
    ) -> Result<asset::Model, EditorToolsError> {
        let model_to_insert = asset::ActiveModel {
            mission_id: Set(payload.mission_id),
            table_name: Set(payload.lua_table_name.clone()),
            path_to_generate: Set(payload.path_to_generate.clone()),
            name: Set(payload.app_name.clone()),
            ..Default::default()
        };
        let res = asset::Entity::insert(model_to_insert)
            .exec_with_returning(&self.db)
            .await?;
        Ok(res)
    }

    pub async fn update_asset(
        &self,
        payload: UpdateFightAssetPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_asset) = asset::Entity::find_by_id(payload.id).one(&self.db).await? {
            let mut model_to_update = existing_asset.into_active_model();
            if let Some(name) = payload.app_name {
                model_to_update.name = Set(name);
            }
            if let Some(path_to_generate) = payload.path_to_generate {
                model_to_update.path_to_generate = Set(path_to_generate);
            }
            if let Some(lua_table_name) = payload.lua_table_name {
                model_to_update.table_name = Set(lua_table_name);
            }
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn delete_asset(&self, id: i32) -> Result<(), EditorToolsError> {
        if let Some(asset) = asset::Entity::find_by_id(id).one(&self.db).await? {
            asset.delete(&self.db).await?;
        }
        Ok(())
    }

    pub async fn get_artifacts_model(
        &self,
        asset_id: i32,
    ) -> Result<Option<artifacts::Model>, EditorToolsError> {
        Ok(artifacts::Entity::find()
            .filter(artifacts::Column::AssetId.eq(asset_id))
            .one(&self.db)
            .await?)
    }

    pub async fn add_artifacts_model(
        &self,
        payload: InitAssetArtifactsDataPayload,
    ) -> Result<artifacts::Model, EditorToolsError> {
        if let Some(asset) = asset::Entity::find_by_id(payload.asset_id).one(&self.db).await? {
            let model_to_insert = artifacts::ActiveModel {
                id: Default::default(),
                asset_id: Set(payload.asset_id),
                update_type: Set(payload.update_type.clone()),
                base_powers: Set(DifficultyMappedValue::default()),
                powers_grow: Set(DifficultyMappedValue::default()),
                required: Set(RequiredArtifacts::default()),
                optional: Set(OptionalArtifacts::default()),
                sheet_name: Set(asset.table_name.clone()),
            };

            let model = model_to_insert.insert(&self.db).await?;
            Ok(model)   
        } else { 
            Err(EditorToolsError::Default)
        }
    }

    pub async fn update_artifacts_generation_type(
        &self,
        payload: UpdateArtifactsGenerationTypePayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.update_type = Set(payload.new_type);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_artifacts_base_generation_power(
        &self,
        payload: UpdateDifficultyBasedPowerPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find()
            .filter(artifacts::Column::AssetId.eq(payload.id))
            .one(&self.db)
            .await?
        {
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
        payload: UpdateDifficultyBasedPowerPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find()
            .filter(artifacts::Column::AssetId.eq(payload.id))
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut powers_grow_to_update = existing_model.powers_grow.clone();
            if let Some(power) = powers_grow_to_update.data.get_mut(&payload.difficulty) {
                *power = payload.value;
            }
            model_to_update.powers_grow = Set(powers_grow_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn add_required_artifact_id(
        &self,
        payload: AddRequiredArtifactPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find()
            .filter(artifacts::Column::AssetId.eq(payload.asset_id))
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut required_artifacts_to_update = existing_model.required.clone();
            required_artifacts_to_update.ids.push(payload.artifact_id);
            model_to_update.required = Set(required_artifacts_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn remove_required_artifact_id(
        &self,
        payload: RemoveRequiredArtifactPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find()
            .filter(artifacts::Column::AssetId.eq(payload.asset_id))
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut required_artifacts_to_update = existing_model.required.clone();
            required_artifacts_to_update
                .ids
                .retain(|id| *id != payload.artifact_id);
            model_to_update.required = Set(required_artifacts_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn add_optional_artifact_id(
        &self,
        payload: AddOptionalArtifactPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find()
            .filter(artifacts::Column::AssetId.eq(payload.asset_id))
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut optional_artifacts_to_update = existing_model.optional.clone();
            if let Some(ids) = optional_artifacts_to_update.values.get_mut(&payload.slot) {
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
        if let Some(existing_model) = artifacts::Entity::find()
            .filter(artifacts::Column::AssetId.eq(payload.asset_id))
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut optional_artifacts_to_update = existing_model.optional.clone();
            if let Some(ids) = optional_artifacts_to_update.values.get_mut(&payload.slot) {
                ids.retain(|id| *id != payload.artifact_id);
            }
            model_to_update.optional = Set(optional_artifacts_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn get_stacks_ids(&self, asset_id: i32) -> Result<Vec<i32>, EditorToolsError> {
        let ids = army_slot::Entity::find()
            .filter(army_slot::Column::AssetId.eq(asset_id))
            .into_partial_model::<ArmySlotId>()
            .all(&self.db)
            .await?;

        Ok(ids.iter().map(|slot| slot.id).collect_vec())
    }

    pub async fn get_stacks(
        &self,
        asset_id: i32,
    ) -> Result<Vec<AssetArmySlotModel>, EditorToolsError> {
        Ok(army_slot::Entity::find()
            .filter(army_slot::Column::AssetId.eq(asset_id))
            .all(&self.db)
            .await?)
    }

    pub async fn add_stack(&self, payload: AddStackPayload) -> Result<i32, EditorToolsError> {
        if let Some(asset) = asset::Entity::find_by_id(payload.asset_id).one(&self.db).await? {
            let stacks_count = army_slot::Entity::find()
                .filter(army_slot::Column::AssetId.eq(payload.asset_id))
                .count(&self.db)
                .await?;
            let model_to_insert = army_slot::ActiveModel {
                asset_id: Set(payload.asset_id),
                selection_type: Set(payload.selection_rule),
                calculation_type: Set(payload.calculation_rule),
                update_type: Set(payload.update_type),
                base_powers: Set(DifficultyMappedValue::default()),
                powers_grow: Set(DifficultyMappedValue::default()),
                towns: Set(CreatureTowns { towns: vec![] }),
                tiers: Set(CreatureTiers { tiers: vec![] }),
                rules: Set(StackGenerationRules { rules: vec![] }),
                specific_creatures: Set(CreatureIds { ids: vec![] }),
                specific_count: Set(DifficultyMappedValue::default()),
                number: Set(stacks_count as i32),
                stat_rules: Set(StackStatsRules { rules: vec![] }),
                sheet_name: Set(asset.table_name.clone()),
                ..Default::default()
            };
            let inserted_model = model_to_insert.insert(&self.db).await?;
            Ok(inserted_model.id)   
        } else {
            Err(EditorToolsError::Default)
        }
    }

    pub async fn get_stack(
        &self,
        stack_id: i32,
    ) -> Result<Option<army_slot::Model>, EditorToolsError> {
        Ok(army_slot::Entity::find_by_id(stack_id)
            .one(&self.db)
            .await?)
    }

    pub async fn delete_stack(&self, stack_id: i32) -> Result<Option<i32>, EditorToolsError> {
        if let Some(stack) = army_slot::Entity::find_by_id(stack_id)
            .one(&self.db)
            .await?
        {
            let asset_id = stack.asset_id;
            stack.delete(&self.db).await?;
            Ok(Some(asset_id))
        } else {
            Ok(None)
        }
    }

    pub async fn update_stack_base_data(
        &self,
        payload: UpdateStackBaseDataPayload,
    ) -> Result<Option<i32>, EditorToolsError> {
        if let Some(stack) = army_slot::Entity::find_by_id(payload.stack_id)
            .one(&self.db)
            .await?
        {
            let asset_id = stack.asset_id;
            let mut model_to_update = stack.into_active_model();
            model_to_update.selection_type = Set(payload.selection_rule);
            model_to_update.calculation_type = Set(payload.calculation_rule);
            model_to_update.update_type = Set(payload.update_type);
            model_to_update.update(&self.db).await?;
            Ok(Some(asset_id))
        } else {
            Ok(None)
        }
    }

    pub async fn update_stack_base_power(
        &self,
        payload: UpdateDifficultyBasedPowerPayload,
    ) -> Result<Option<i32>, EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.id)
            .one(&self.db)
            .await?
        {
            let asset_id = existing_model.asset_id;
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut base_powers_to_update = existing_model.base_powers.clone();
            if let Some(power) = base_powers_to_update.data.get_mut(&payload.difficulty) {
                *power = payload.value;
            }
            model_to_update.base_powers = Set(base_powers_to_update);
            model_to_update.update(&self.db).await?;
            Ok(Some(asset_id))
        } else {
            Ok(None)
        }
    }

    pub async fn update_stack_power_grow(
        &self,
        payload: UpdateDifficultyBasedPowerPayload,
    ) -> Result<Option<i32>, EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.id)
            .one(&self.db)
            .await?
        {
            let asset_id = existing_model.asset_id;
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut powers_grow_to_update = existing_model.powers_grow.clone();
            if let Some(power) = powers_grow_to_update.data.get_mut(&payload.difficulty) {
                *power = payload.value;
            }
            model_to_update.powers_grow = Set(powers_grow_to_update);
            model_to_update.update(&self.db).await?;
            Ok(Some(asset_id))
        } else {
            Ok(None)
        }
    }

    pub async fn update_stack_creature_count(
        &self,
        payload: UpdateDifficultyBasedPowerPayload,
    ) -> Result<Option<i32>, EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.id)
            .one(&self.db)
            .await?
        {
            let asset_id = existing_model.asset_id;
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut creature_counts_to_update = existing_model.specific_count.clone();
            if let Some(power) = creature_counts_to_update.data.get_mut(&payload.difficulty) {
                *power = payload.value;
            }
            model_to_update.specific_count = Set(creature_counts_to_update);
            model_to_update.update(&self.db).await?;
            Ok(Some(asset_id))
        } else {
            Ok(None)
        }
    }

    pub async fn update_stack_specific_creatures(
        &self,
        payload: UpdateStackSpecificCreaturesPayload,
    ) -> Result<Option<i32>, EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.stack_id)
            .one(&self.db)
            .await?
        {
            let asset_id = existing_model.asset_id;
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.specific_creatures = Set(CreatureIds {
                ids: payload.creatures,
            });
            model_to_update.update(&self.db).await?;
            Ok(Some(asset_id))
        } else {
            Ok(None)
        }
    }

    pub async fn update_stack_towns(
        &self,
        payload: UpdateStackTownsPayload,
    ) -> Result<Option<i32>, EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.stack_id)
            .one(&self.db)
            .await?
        {
            let asset_id = existing_model.asset_id;
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.towns = Set(CreatureTowns {
                towns: payload.towns,
            });
            model_to_update.update(&self.db).await?;
            Ok(Some(asset_id))
        } else {
            Ok(None)
        }
    }

    pub async fn update_stack_tiers(
        &self,
        payload: UpdateStackTiersPayload,
    ) -> Result<Option<i32>, EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.stack_id)
            .one(&self.db)
            .await?
        {
            let asset_id = existing_model.asset_id;
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.tiers = Set(CreatureTiers {
                tiers: payload.tiers,
            });
            model_to_update.update(&self.db).await?;
            Ok(Some(asset_id))
        } else {
            Ok(None)
        }
    }

    pub async fn update_stack_rules(
        &self,
        payload: UpdateGenerationRulesPayload,
    ) -> Result<Option<i32>, EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.stack_id)
            .one(&self.db)
            .await?
        {
            let asset_id = existing_model.asset_id;
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.rules = Set(StackGenerationRules {
                rules: payload.rules,
            });
            model_to_update.update(&self.db).await?;
            Ok(Some(asset_id))
        } else {
            Ok(None)
        }
    }

    pub async fn update_stat_rules(
        &self,
        payload: UpdateStatRulesPayload
    ) -> Result<Option<i32>, EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.stack_id).one(&self.db).await? {
            let asset_id = existing_model.asset_id;
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.stat_rules = Set(StackStatsRules {rules: payload.rules});
            model_to_update.update(&self.db).await?;
            Ok(Some(asset_id))
        } else {
            Ok(None)
        }
    }

    pub async fn get_stacks_count_for_asset(
        &self,
        asset_id: i32,
    ) -> Result<u64, EditorToolsError> {
        Ok(army_slot::Entity::find()
            .filter(army_slot::Column::AssetId.eq(asset_id))
            .count(&self.db)
            .await?)
    }
}