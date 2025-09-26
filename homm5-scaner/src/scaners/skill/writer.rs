use std::fs::File;

use sea_orm::{
    Condition, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter
};
use zip::ZipWriter;

use crate::{core::Output, error::ScanerError};

pub struct SkillDataOutput<'a> {
    entities: Vec<super::model::Model>,
    db: &'a DatabaseConnection,
}

impl<'a> SkillDataOutput<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        SkillDataOutput {
            entities: vec![],
            db,
        }
    }
}

impl<'a> Output for SkillDataOutput<'a> {
    type Input = super::model::Model;

    fn output_single(&mut self, object: Self::Input) -> Result<(), ScanerError> {
        self.entities.push(object);
        Ok(())
    }

    async fn finish_output(&self, _zip_writer: &mut ZipWriter<File>) -> Result<(), ScanerError> {
        super::model::Entity::delete_many()
            .filter(Condition::all())
            .exec(self.db)
            .await?;
        super::model::Entity::insert_many(
            self.entities
                .iter()
                .map(|model| model.clone().into_active_model()),
        )
        .exec(self.db)
        .await?;

        // let mut script_file = String::from("MCCS_CREATURE_GENERATED_TABLE = {\n");
        // for model in &self.entities {
        //     script_file += &model.to_lua_string();
        // }
        // script_file.push_str("}");
        // zip_writer.start_file("scripts/generated/creatures.lua", FileOptions::default())?;
        // zip_writer.write_all(script_file.as_bytes())?;

        Ok(())
    }
}
