use std::{fs::File, io::Write};

use itertools::Itertools;
use sea_orm::{
    DatabaseConnection, EntityTrait, IntoActiveModel, Iterable, TransactionTrait,
    sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use zip::{ZipWriter, write::FileOptions};

use crate::{
    core::{Output, ToJsonCompatibleString, ToLua},
    error::ScanerError, prelude::ArtifactDBModel,
};

use super::model::Column;

pub struct ArtifactDataOutput<'a> {
    entities: Vec<super::model::Model>,
    db: &'a DatabaseConnection,
}

impl<'a> ArtifactDataOutput<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        ArtifactDataOutput {
            entities: vec![],
            db,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCompatibleModel<'a> {
    #[serde(rename(serialize = "ArtifactType"))]
    pub id: i32,
    #[serde(rename(serialize = "ArtifactCategory"))]
    pub class: &'a str,
    #[serde(rename(serialize = "ArtifactSlot"))]
    pub slot: &'a str,
    #[serde(rename(serialize = "Cost"))]
    pub cost: i32
}

impl<'a> From<&'a ArtifactDBModel> for JsonCompatibleModel<'a> {
    fn from(value: &'a ArtifactDBModel) -> Self {
        JsonCompatibleModel { 
            id: value.id, 
            class: value.class.to_json_compatible_repr(), 
            slot: value.slot.to_json_compatible_repr(), 
            cost: value.cost 
        }
    }
}

impl<'a> Output for ArtifactDataOutput<'a> {
    type Input = super::model::Model;

    fn output_single(&mut self, object: Self::Input) -> Result<(), ScanerError> {
        self.entities.push(object);
        Ok(())
    }

    async fn finish_output(&self, zip_writer: &mut ZipWriter<File>) -> Result<(), ScanerError> {
        let transaction = self.db.begin().await?;
        let on_conflict = OnConflict::new()
            .update_columns(
                super::model::Column::iter()
                    .filter_map(|column| match column {
                        Column::Id => None,
                        _ => Some(column),
                    })
                    .collect::<Vec<super::model::Column>>(),
            )
            .to_owned();
        for entity in &self.entities {
            let active_model = entity.clone().into_active_model();
            super::model::Entity::insert(active_model)
                .on_conflict(on_conflict.clone())
                .exec(&transaction)
                .await?;
        }
        transaction.commit().await?;

        let mut script_file = String::from("MCCS_ARTIFACTS_GENERATED_TABLE = {\n");
        for model in &self.entities {
            script_file += &model.to_lua_string();
        }
        script_file.push_str("}");
        zip_writer.start_file("scripts/generated/artifacts.lua", FileOptions::default())?;
        zip_writer.write_all(script_file.as_bytes())?;

        let json_models = self.entities.iter()
            .map(|m| {
                JsonCompatibleModel::from(m)
            })
            .collect_vec();


        let json_string = serde_json::to_string_pretty(&json_models)?;
        let mut file = std::fs::File::create("D:\\arts.json")?;
        file.write_all(json_string.as_bytes())?;

        Ok(())
    }
}
