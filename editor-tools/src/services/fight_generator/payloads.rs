use homm5_scaner::prelude::{ArtifactSlotType, TownExtended};
use crate::prelude::{DifficultyMappedValue, OptionalArtifacts, RequiredArtifacts, StackGenerationRule, StatParam, StatSelectionRule};
use crate::services::fight_generator::models::army_slot::{CalculationRule, SelectionType, StackStatsRuleModel, StackStatsRules};
use crate::services::fight_generator::models::common::AssetUpdateType;
use super::prelude::DifficultyType;

pub struct InitFightAssetPayload {
    pub app_name: String,
    pub mission_id: i32,
    pub path_to_generate: String,
    pub lua_table_name: String
}

#[derive(Default)]
pub struct UpdateFightAssetPayload {
    pub id: i32,
    pub app_name: Option<String>,
    pub path_to_generate: Option<String>,
    pub lua_table_name: Option<String>
}

impl UpdateFightAssetPayload {
    pub fn new(id: i32) -> Self {
        UpdateFightAssetPayload {
            id,
            ..Default::default()
        }
    }

    pub fn with_app_name(mut self, app_name: String) -> Self {
        self.app_name = Some(app_name);
        self
    }

    pub fn with_path_to_generate(mut self, path: String) -> Self {
        self.path_to_generate = Some(path);
        self
    }

    pub fn with_lua_table_name(mut self, name: String) -> Self {
        self.lua_table_name = Some(name);
        self
    }
}

#[derive(Debug)]
pub struct InitAssetArtifactsDataPayload {
    pub asset_id: i32,
    pub update_type: AssetUpdateType
}

pub struct UpdateArtifactsGenerationTypePayload {
    pub id: i32,
    pub new_type: AssetUpdateType,
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
    pub asset_id: i32,
    pub selection_rule: SelectionType,
    pub calculation_rule: CalculationRule,
    pub update_type: AssetUpdateType
}

pub struct UpdateStackBaseDataPayload {
    pub stack_id: i32,
    pub selection_rule: SelectionType,
    pub calculation_rule: CalculationRule,
    pub update_type: AssetUpdateType
}

pub struct UpdateStackSpecificCreaturesPayload {
    pub stack_id: i32,
    pub creatures: Vec<i32>,
}

pub struct UpdateStackTownsPayload {
    pub stack_id: i32,
    pub towns: Vec<TownExtended>,
}

pub struct UpdateStackTiersPayload {
    pub stack_id: i32,
    pub tiers: Vec<i32>,
}

pub struct UpdateGenerationRulesPayload {
    pub stack_id: i32,
    pub rules: Vec<StackGenerationRule>,
}

pub struct AddStatSelectionRulePayload {
    pub stack_id: i32,
    pub rule: StackStatsRuleModel,
}

pub struct UpdateStatParamsPayload {
    pub element_id: i32,
    pub params: Vec<StatParam>,
}

#[derive(Default)]
pub struct UpdateGenerationStatElementPayload {
    pub stack_id: i32,
    pub element_id: i32,
    pub rule: Option<StatSelectionRule>,
    pub priority: Option<i32>,
    pub stats: Option<Vec<StatParam>>,
}

impl UpdateGenerationStatElementPayload {
    pub fn new(stack_id: i32, element_id: i32) -> Self {
        UpdateGenerationStatElementPayload {
            stack_id,
            element_id,
            ..Default::default()
        }
    }

    pub fn with_rule(mut self, rule: StatSelectionRule) -> Self {
        self.rule = Some(rule);
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn with_stats(mut self, stats: Vec<StatParam>) -> Self {
        self.stats = Some(stats);
        self
    }
}

#[derive(Default)]
pub struct UpdateArtifactsAssetPayload {
    pub asset_id: i32,
    pub required: Option<RequiredArtifacts>,
    pub optional: Option<OptionalArtifacts>,
    pub base_values: Option<DifficultyMappedValue>,
    pub values_grow: Option<DifficultyMappedValue>,
}

impl UpdateArtifactsAssetPayload {
    pub fn new(asset_id: i32) -> Self {
        UpdateArtifactsAssetPayload {
            asset_id,
            ..Default::default()
        }
    }

    pub fn with_base_values(mut self, base: DifficultyMappedValue) -> Self {
        self.base_values = Some(base);
        self
    }

    pub fn with_values_grow(mut self, grow: DifficultyMappedValue) -> Self {
        self.values_grow = Some(grow);
        self
    }

    pub fn with_optional(mut self, optional: OptionalArtifacts) -> Self {
        self.optional = Some(optional);
        self
    }

    pub fn with_required(mut self, required: RequiredArtifacts) -> Self {
        self.required = Some(required);
        self
    }
}