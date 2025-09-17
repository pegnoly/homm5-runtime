use std::fs::File;

use sea_orm::{
    DatabaseConnection, EntityTrait, IntoActiveModel, Iterable, TransactionTrait,
    sea_query::OnConflict,
};
use zip::ZipWriter;

use crate::{core::Output, error::ScanerError};

use super::model::Column;

pub struct HeroClassDataOutput<'a> {
    entities: Vec<super::model::Model>,
    db: &'a DatabaseConnection,
}

impl<'a> HeroClassDataOutput<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        HeroClassDataOutput {
            entities: vec![],
            db,
        }
    }
}

impl<'a> Output for HeroClassDataOutput<'a> {
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
                    .filter(|column| !matches!(column, Column::Id))
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

        // let mut script_file = String::from("MCCS_CREATURE_GENERATED_TABLE = {\n");
        // for model in &self.entities {
        //     script_file += &model.to_lua_string();
        // }
        // script_file.push_str("}");
        // zip_writer.start_file("scripts/generated/creatures.lua", FileOptions::default())?;
        // zip_writer.write_all(script_file.as_bytes())?;

        // let mut json_file = std::fs::File::create("D:\\creatures.json")?;
        // let json_string = serde_json::to_string_pretty(&self.entities)?;
        // json_file.write_all(json_string.as_bytes())?;

        Ok(())
    }
}
