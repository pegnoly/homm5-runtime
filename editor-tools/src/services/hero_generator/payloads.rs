use homm5_scaner::prelude::ArtifactSlotType;

use super::{models::common::AssetGenerationType, prelude::DifficultyType};

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

pub struct UpdateArtifactsGenerationPowerPayload {
    pub id: i32,
    pub difficulty: DifficultyType,
    pub value: i32
}

pub struct AddOptionalArtifactPayload {
    pub hero_id: i32,
    pub slot: ArtifactSlotType,
    pub artifact_id: i32,
}

pub struct RemoveOptionalArtifactPayload {
    pub hero_id: i32,
    pub slot: ArtifactSlotType,
    pub artifact_id: i32,
}
