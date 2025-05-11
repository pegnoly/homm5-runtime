use homm5_scaner::prelude::ArtifactSlotType;

use super::models::generatable_hero::ArtifactsGenerationCostType;

pub struct InitGeneratableHeroPayload {
    pub path_to_generate: String,
    pub lua_table_name: String
}

pub struct UpdateArtifactCostTypePayload {
    pub id: i32,
    pub new_type: ArtifactsGenerationCostType
}

pub struct AddOptionalArtifactPayload {
    pub hero_id: i32,
    pub slot: ArtifactSlotType,
    pub artifact_id: i32
}

pub struct RemoveOptionalArtifactPayload {
    pub hero_id: i32,
    pub slot: ArtifactSlotType,
    pub artifact_id: i32
}