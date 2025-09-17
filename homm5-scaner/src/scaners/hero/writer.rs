use std::{fs::File, io::Write};

use sea_orm::{Condition, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use zip::{ZipWriter, write::FileOptions};

use crate::{
    core::{Output, ToLua},
    error::ScanerError,
};

pub struct HeroDataOutput<'a> {
    entities: Vec<super::model::Model>,
    db: &'a DatabaseConnection,
}

impl<'a> HeroDataOutput<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        HeroDataOutput {
            entities: vec![],
            db,
        }
    }
}

impl<'a> Output for HeroDataOutput<'a> {
    type Input = super::model::Model;

    fn output_single(&mut self, object: Self::Input) -> Result<(), ScanerError> {
        self.entities.push(object);
        Ok(())
    }

    async fn finish_output(&self, zip_writer: &mut ZipWriter<File>) -> Result<(), ScanerError> {
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
        let mut script_file = String::from("MCCS_GENERATED_HEROES_TABLE = {\n");
        for model in &self.entities {
            script_file += &model.to_lua_string();
        }
        script_file.push('}');
        zip_writer.start_file("scripts/generated/heroes.lua", FileOptions::default())?;
        zip_writer.write_all(script_file.as_bytes())?;

        let mut json_file = std::fs::File::create("D:\\heroes.json")?;
        let json_string = serde_json::to_string_pretty(&self.entities)?;
        json_file.write_all(json_string.as_bytes())?;

        Ok(())
    }
}
