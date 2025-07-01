use std::{fs::File, io::Write};

use sea_orm::{
    DatabaseConnection, EntityTrait, IntoActiveModel, Iterable, TransactionTrait,
    sea_query::OnConflict,
};
use zip::{ZipWriter, write::FileOptions};

use crate::{
    core::{Output, ToLua},
    error::ScanerError,
};

use super::model::Column;

pub struct SpellDataOutput<'a> {
    entities: Vec<super::model::Model>,
    db: &'a DatabaseConnection,
}

impl<'a> SpellDataOutput<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        SpellDataOutput {
            entities: vec![],
            db,
        }
    }
}

impl<'a> Output for SpellDataOutput<'a> {
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

        let mut script_file = String::from("MCCS_SPELL_GENERATED_TABLE = {\n");
        for model in &self.entities {
            script_file += &model.to_lua_string();
        }
        script_file.push_str("}");
        zip_writer.start_file("scripts/generated/spells.lua", FileOptions::default())?;
        zip_writer.write_all(script_file.as_bytes())?;

        Ok(())
    }
}
