use homm5_scaner::prelude::{ArtifactSlotType, Town};
use sea_orm::prelude::Uuid;

use super::{
    models::{
        army_slot::{
            ArmyGenerationRuleParam, ArmySlotStackCountGenerationMode,
            ArmySlotStackUnitGenerationMode,
        },
        common::AssetGenerationType,
        stat_generation::{ArmyGenerationStatParam, ArmyGenerationStatRule},
    },
    prelude::DifficultyType,
};

pub struct InitFightAssetPayload {
    pub name: String,
    pub mission_id: i32,
    pub path_to_generate: String,
    pub lua_table_name: String,
}

pub struct InitAssetArtifactsDataPayload {
    pub asset_id: Uuid,
    pub generation_type: AssetGenerationType,
}

pub struct UpdateArtifactsGenerationTypePayload {
    pub id: i32,
    pub new_type: AssetGenerationType,
}

pub struct UpdateDifficultyBasedPowerPayload {
    pub id: i32,
    pub difficulty: DifficultyType,
    pub value: i32,
}

pub struct AddOptionalArtifactPayload {
    pub asset_id: i32,
    pub slot: ArtifactSlotType,
    pub artifact_id: i32,
}

pub struct RemoveOptionalArtifactPayload {
    pub asset_id: i32,
    pub slot: ArtifactSlotType,
    pub artifact_id: i32,
}

pub struct AddRequiredArtifactPayload {
    pub asset_id: i32,
    pub artifact_id: i32,
}

pub struct RemoveRequiredArtifactPayload {
    pub asset_id: i32,
    pub artifact_id: i32,
}

pub struct AddStackPayload {
    pub asset_id: Uuid,
    pub unit_generation_type: ArmySlotStackUnitGenerationMode,
    pub count_generation_type: ArmySlotStackCountGenerationMode,
    pub power_based_generation_type: Option<AssetGenerationType>,
}

impl AddStackPayload {
    pub fn new(
        asset_id: Uuid,
        unit_generation_type: ArmySlotStackUnitGenerationMode,
        count_generation_type: ArmySlotStackCountGenerationMode,
    ) -> Self {
        AddStackPayload {
            asset_id,
            unit_generation_type,
            count_generation_type,
            power_based_generation_type: None,
        }
    }

    pub fn with_power_based_generation_type(
        mut self,
        generation_type: AssetGenerationType,
    ) -> Self {
        self.power_based_generation_type = Some(generation_type);
        self
    }
}

pub struct UpdateStackBaseDataPayload {
    pub stack_id: i32,
    pub unit_generation_type: ArmySlotStackUnitGenerationMode,
    pub count_generation_type: ArmySlotStackCountGenerationMode,
    pub power_based_generation_type: AssetGenerationType
}

pub struct UpdateStackConcreteCreaturesPayload {
    pub stack_id: i32,
    pub creatures: Vec<i32>,
}

pub struct UpdateStackTownsPayload {
    pub stack_id: i32,
    pub towns: Vec<Town>,
}

pub struct UpdateStackTiersPayload {
    pub stack_id: i32,
    pub tiers: Vec<i32>,
}

pub struct UpdateGenerationRulesPayload {
    pub stack_id: i32,
    pub rules: Vec<ArmyGenerationRuleParam>,
}

pub struct AddGenerationStatElementPayload {
    pub stack_id: i32,
    pub rule: ArmyGenerationStatRule,
}

pub struct UpdateGenerationStatParamsPayload {
    pub element_id: i32,
    pub params: Vec<ArmyGenerationStatParam>,
}

#[derive(Default)]
pub struct UpdateGenerationStatElementPayload {
    pub element_id: i32,
    pub rule: Option<ArmyGenerationStatRule>,
    pub priority: Option<i32>,
    pub stats: Option<Vec<ArmyGenerationStatParam>>,
}

impl UpdateGenerationStatElementPayload {
    pub fn new(id: i32) -> Self {
        UpdateGenerationStatElementPayload {
            element_id: id,
            ..Default::default()
        }
    }

    pub fn with_rule(mut self, rule: ArmyGenerationStatRule) -> Self {
        self.rule = Some(rule);
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn with_stats(mut self, stats: Vec<ArmyGenerationStatParam>) -> Self {
        self.stats = Some(stats);
        self
    }
}
