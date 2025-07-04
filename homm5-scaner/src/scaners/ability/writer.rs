use std::fs::File;

use sea_orm::{
    DatabaseConnection, EntityTrait, IntoActiveModel, Iterable, TransactionTrait,
    sea_query::OnConflict,
};
use zip::ZipWriter;

use crate::{
    core::Output,
    error::ScanerError,
};

use super::model::Column;

pub struct AbilityDataOutput<'a> {
    entities: Vec<super::model::Model>,
    db: &'a DatabaseConnection,
}

impl<'a> AbilityDataOutput<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        AbilityDataOutput {
            entities: vec![],
            db,
        }
    }
}

impl<'a> Output for AbilityDataOutput<'a> {
    type Input = super::model::Model;

    fn output_single(&mut self, object: Self::Input) -> Result<(), ScanerError> {
        self.entities.push(object);
        Ok(())
    }

    async fn finish_output(&self, _zip_writer: &mut ZipWriter<File>) -> Result<(), ScanerError> {
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

        // let mut script_file = String::from("MCCS_ARTIFACTS_GENERATED_TABLE = {\n");
        // for model in &self.entities {
        //     script_file += &model.to_lua_string();
        // }
        // script_file.push_str("}");
        // zip_writer.start_file("scripts/generated/artifacts.lua", FileOptions::default())?;
        // zip_writer.write_all(script_file.as_bytes())?;

        Ok(())
    }
}
