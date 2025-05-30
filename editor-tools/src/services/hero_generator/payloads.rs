use homm5_scaner::prelude::{ArtifactSlotType, Town};

use super::{models::{army_slot::{ArmyGenerationRuleParam, ArmySlotStackCountGenerationMode, ArmySlotStackUnitGenerationMode}, common::AssetGenerationType, stat_generation::{ArmyGenerationStatParam, ArmyGenerationStatRule}}, prelude::DifficultyType};

pub struct InitGeneratableHeroPayload {
    pub name: String,
    pub path_to_generate: String,
    pub lua_table_name: String,
}

pub struct InitAssetArtifactsDataPayload {
    pub asset_id: i32,
    pub generation_type: AssetGenerationType,
}

pub struct UpdateArtifactsGenerationTypePayload {
    pub id: i32,
    pub new_type: AssetGenerationType,
}

pub struct UpdateDifficultyBasedPowerPayload {
    pub id: i32,
    pub difficulty: DifficultyType,
    pub value: i32
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
    pub asset_id: i32,
    pub unit_generation_type: ArmySlotStackUnitGenerationMode,
    pub count_generation_type: ArmySlotStackCountGenerationMode,
    pub power_based_generation_type: Option<AssetGenerationType>
}

impl AddStackPayload {
    pub fn new(asset_id: i32, unit_generation_type: ArmySlotStackUnitGenerationMode, count_generation_type: ArmySlotStackCountGenerationMode) -> Self {
        AddStackPayload {
            asset_id,
            unit_generation_type,
            count_generation_type,
            power_based_generation_type: None
        }
    }

    pub fn with_power_based_generation_type(mut self, generation_type: AssetGenerationType) -> Self {
        self.power_based_generation_type = Some(generation_type);
        self
    }
}

pub struct UpdateStackConcreteCreaturePayload {
    pub stack_id: i32,
    pub creature: i32
}

pub struct UpdateStackCreatureTownPayload {
    pub stack_id: i32,
    pub town: Town
}

pub struct UpdateStackCreatureTierPayload {
    pub stack_id: i32,
    pub tier: i32
}

pub struct UpdateGenerationRulesPayload {
    pub stack_id: i32,
    pub rule: ArmyGenerationRuleParam
}

pub struct AddGenerationStatElementPayload {
    pub stack_id: i32,
    pub rule: ArmyGenerationStatRule
}

pub struct UpdateGenerationStatParamsPayload {
    pub element_id: i32,
    pub params: Vec<ArmyGenerationStatParam>
}

#[derive(Default)]
pub struct UpdateGenerationStatElementPayload {
    pub element_id: i32,
    pub rule: Option<ArmyGenerationStatRule>,
    pub priority: Option<i32>
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
}