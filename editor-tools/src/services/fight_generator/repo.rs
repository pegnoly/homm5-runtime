use super::{
    models::{
        army_slot::{self, ArmySlotGenerationRule, ArmySlotId},
        artifacts::{self, OptionalArtifacts, RequiredArtifacts},
        asset,
        common::{AssetGenerationType, DifficultyMappedValue},
        stat_generation::{self, ArmyGenerationStats},
    },
    payloads::{AddOptionalArtifactPayload, RemoveOptionalArtifactPayload},
    prelude::{
        AddGenerationStatElementPayload, AddRequiredArtifactPayload, AddStackPayload,
        InitAssetArtifactsDataPayload, RemoveRequiredArtifactPayload,
        UpdateArtifactsGenerationTypePayload, UpdateDifficultyBasedPowerPayload,
        UpdateGenerationRulesPayload, UpdateGenerationStatElementPayload,
    },
};
use crate::{
    error::EditorToolsError,
    prelude::{
        ArmyStatGenerationModel, AssetArmySlotModel, InitFightAssetPayload, UpdateFightAssetPayload, UpdateStackBaseDataPayload, UpdateStackConcreteCreaturesPayload, UpdateStackTiersPayload, UpdateStackTownsPayload
    },
    services::fight_generator::models::army_slot::{CreatureIds, CreatureTiers, CreatureTowns},
};
use itertools::Itertools;
use sea_orm::{
    sqlx::{types::uuid, SqlitePool}, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, SqlxSqlitePoolConnection
};
use uuid::Uuid;

pub struct FightGeneratorRepo {
    db: DatabaseConnection,
}

impl FightGeneratorRepo {
    pub fn new(pool: SqlitePool) -> Self {
        FightGeneratorRepo {
            db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)),
        }
    }

    pub async fn get_all_assets(
        &self,
        mission_id: i32,
    ) -> Result<Vec<asset::Model>, EditorToolsError> {
        Ok(asset::Entity::find()
            .filter(asset::Column::MissionId.eq(mission_id))
            .all(&self.db)
            .await?)
    }

    pub async fn get_asset(&self, id: Uuid) -> Result<Option<asset::Model>, EditorToolsError> {
        Ok(asset::Entity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn init_new_asset(
        &self,
        payload: InitFightAssetPayload,
    ) -> Result<Uuid, EditorToolsError> {
        let model_to_insert = asset::ActiveModel {
            mission_id: Set(payload.mission_id),
            table_name: Set(payload.lua_table_name.clone()),
            path_to_generate: Set(payload.path_to_generate.clone()),
            name: Set(payload.name.clone()),
            id: Set(Uuid::new_v4()),
            sheet_id: Set(Some(payload.sheet_id))
        };
        let res = asset::Entity::insert(model_to_insert).exec_with_returning(&self.db).await?;
        Ok(res.id)
    }

    pub async fn update_asset(&self, payload: UpdateFightAssetPayload) -> Result<(), EditorToolsError> {
        if let Some(existing_asset) = asset::Entity::find_by_id(payload.id).one(&self.db).await? {
            let mut model_to_update = existing_asset.into_active_model();
            if let Some(name) = payload.name {
                model_to_update.name = Set(name);
            }
            if let Some(path_to_generate) = payload.path_to_generate {
                model_to_update.path_to_generate = Set(path_to_generate);
            }
            if let Some(lua_table_name) = payload.lua_table_name {
                model_to_update.table_name = Set(lua_table_name);
            }
            if let Some(sheet_id) = payload.sheet_id {
                model_to_update.sheet_id = Set(Some(sheet_id));
            }
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn delete_asset(
        &self,
        id: Uuid
    ) -> Result<(), EditorToolsError> {
        if let Some(asset) = asset::Entity::find_by_id(id).one(&self.db).await? {
            asset.delete(&self.db).await?;
        }
        Ok(())
    }

    pub async fn get_artifacts_model(
        &self,
        asset_id: Uuid,
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
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.generation_type = Set(payload.new_type);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_artifacts_base_generation_power(
        &self,
        payload: UpdateDifficultyBasedPowerPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.id)
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
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.id)
            .one(&self.db)
            .await?
        {
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

    pub async fn add_required_artifact_id(
        &self,
        payload: AddRequiredArtifactPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.asset_id)
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
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.asset_id)
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
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.asset_id)
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
        if let Some(existing_model) = artifacts::Entity::find_by_id(payload.asset_id)
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

    pub async fn get_stacks_ids(&self, asset_id: Uuid) -> Result<Vec<i32>, EditorToolsError> {
        let ids = army_slot::Entity::find()
            .filter(army_slot::Column::AssetId.eq(asset_id))
            .into_partial_model::<ArmySlotId>()
            .all(&self.db)
            .await?;

        Ok(ids.iter().map(|slot| slot.id).collect_vec())
    }

    pub async fn get_stacks(
        &self,
        asset_id: Uuid,
    ) -> Result<Vec<AssetArmySlotModel>, EditorToolsError> {
        Ok(army_slot::Entity::find()
            .filter(army_slot::Column::AssetId.eq(asset_id))
            .all(&self.db)
            .await?)
    }

    pub async fn add_stack(&self, payload: AddStackPayload) -> Result<i32, EditorToolsError> {
        let model_to_insert = army_slot::ActiveModel {
            asset_id: Set(payload.asset_id),
            type_generation_mode: Set(payload.unit_generation_type),
            count_generation_mode: Set(payload.count_generation_type),
            power_based_generation_type: Set(payload
                .power_based_generation_type
                .unwrap_or(AssetGenerationType::Static)),
            base_powers: Set(DifficultyMappedValue::default()),
            powers_grow: Set(DifficultyMappedValue::default()),
            towns: Set(CreatureTowns { towns: vec![] }),
            tiers: Set(CreatureTiers { tiers: vec![] }),
            generation_rule: Set(ArmySlotGenerationRule { params: vec![] }),
            concrete_creatures: Set(CreatureIds { ids: vec![] }),
            concrete_count: Set(DifficultyMappedValue::default()),
            ..Default::default()
        };
        let inserted_model = model_to_insert.insert(&self.db).await?;
        Ok(inserted_model.id)
    }

    pub async fn get_stack(
        &self,
        stack_id: i32,
    ) -> Result<Option<army_slot::Model>, EditorToolsError> {
        Ok(army_slot::Entity::find_by_id(stack_id)
            .one(&self.db)
            .await?)
    }

    pub async fn delete_stack(
        &self,
        stack_id: i32
    ) -> Result<(), EditorToolsError> {
        if let Some(stack) = army_slot::Entity::find_by_id(stack_id).one(&self.db).await? {
            stack.delete(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_stack_base_data(
        &self,
        payload: UpdateStackBaseDataPayload
    ) -> Result<(), EditorToolsError> {
        if let Some(stack) = army_slot::Entity::find_by_id(payload.stack_id).one(&self.db).await? {
            let mut model_to_update = stack.into_active_model();
            model_to_update.type_generation_mode = Set(payload.unit_generation_type);
            model_to_update.count_generation_mode = Set(payload.count_generation_type);
            model_to_update.power_based_generation_type = Set(payload.power_based_generation_type);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_stack_base_power(
        &self,
        payload: UpdateDifficultyBasedPowerPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.id)
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

    pub async fn update_stack_power_grow(
        &self,
        payload: UpdateDifficultyBasedPowerPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.id)
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

    pub async fn update_stack_creature_count(
        &self,
        payload: UpdateDifficultyBasedPowerPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.clone().into_active_model();
            let mut creature_counts_to_update = existing_model.concrete_count.clone();
            if let Some(power) = creature_counts_to_update.data.get_mut(&payload.difficulty) {
                *power = payload.value;
            }
            model_to_update.concrete_count = Set(creature_counts_to_update);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_stack_concrete_creatures(
        &self,
        payload: UpdateStackConcreteCreaturesPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.stack_id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.concrete_creatures = Set(CreatureIds {
                ids: payload.creatures,
            });
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_stack_towns(
        &self,
        payload: UpdateStackTownsPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.stack_id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.towns = Set(CreatureTowns {
                towns: payload.towns,
            });
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_stack_tiers(
        &self,
        payload: UpdateStackTiersPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.stack_id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.tiers = Set(CreatureTiers {
                tiers: payload.tiers,
            });
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_stack_rules(
        &self,
        payload: UpdateGenerationRulesPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = army_slot::Entity::find_by_id(payload.stack_id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.into_active_model();
            model_to_update.generation_rule = Set(ArmySlotGenerationRule {
                params: payload.rules,
            });
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn get_stat_generation_elements(
        &self,
        stack_id: i32,
    ) -> Result<Vec<stat_generation::Model>, EditorToolsError> {
        Ok(stat_generation::Entity::find()
            .filter(stat_generation::Column::StackId.eq(stack_id))
            .all(&self.db)
            .await?)
    }

    pub async fn add_stat_generation_element(
        &self,
        payload: AddGenerationStatElementPayload,
    ) -> Result<stat_generation::Model, EditorToolsError> {
        let model_to_insert = stat_generation::ActiveModel {
            stack_id: Set(payload.stack_id),
            priority: Set(0),
            rule: Set(payload.rule),
            stats: Set(ArmyGenerationStats { values: vec![] }),
            ..Default::default()
        };
        let model = model_to_insert.insert(&self.db).await?;
        Ok(model)
    }

    pub async fn delete_stat_generation_element(&self, id: i32) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = stat_generation::Entity::find_by_id(id)
            .one(&self.db)
            .await?
        {
            existing_model.delete(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_stat_generation_element(
        &self,
        payload: UpdateGenerationStatElementPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_model) = stat_generation::Entity::find_by_id(payload.element_id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_model.into_active_model();
            if let Some(priority) = payload.priority {
                model_to_update.priority = Set(priority);
            }
            if let Some(rule) = payload.rule {
                model_to_update.rule = Set(rule);
            }
            if let Some(stats) = payload.stats {
                model_to_update.stats = Set(ArmyGenerationStats { values: stats });
            }
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn get_all_stat_elements_for_stacks(&self, stacks_ids: Vec<i32>) -> Result<Vec<ArmyStatGenerationModel>, EditorToolsError> {
        Ok(stat_generation::Entity::find().filter(stat_generation::Column::StackId.is_in(stacks_ids)).all(&self.db).await?)
    }
}
