use homm5_scaner::prelude::{ArtifactSlotType, Town};

use super::{models::{army_slot::ArmyGenerationRuleParam, common::AssetGenerationType}, prelude::DifficultyType};

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
    pub generation_type: AssetGenerationType
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