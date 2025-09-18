use homm5_scaner::prelude::ArtifactSlotType;
use sea_orm::{FromJsonQueryResult, Iterable, prelude::*};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str};

use super::common::{AssetGenerationType, DifficultyMappedValue};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "generated_artifacts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub asset_id: Uuid,
    pub generation_type: AssetGenerationType,
    pub base_powers: DifficultyMappedValue,
    pub powers_grow: Option<DifficultyMappedValue>,
    pub required: RequiredArtifacts,
    pub optional: OptionalArtifacts,
    pub sheet_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct RequiredArtifacts {
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromJsonQueryResult, PartialEq, Eq)]
pub struct OptionalArtifacts {
    pub values: HashMap<ArtifactSlotType, Vec<i32>>,
}

impl Default for OptionalArtifacts {
    fn default() -> Self {
        OptionalArtifacts {
            values: HashMap::from_iter(ArtifactSlotType::iter().map(|s| (s, vec![]))),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
