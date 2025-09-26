use std::{fs::File, io::Write};

use itertools::Itertools;
use sea_orm::{
    Condition, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter
};
use serde::{Deserialize, Serialize};
use zip::{ZipWriter, write::FileOptions};

use crate::{
    core::{Output, ToJsonCompatibleString, ToLua},
    error::ScanerError,
    scaners::spell::model::{MagicSchoolType, Model},
};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCompatibleModel<'a> {
    #[serde(rename(serialize = "SpellType"))]
    pub id: i32,
    #[serde(rename(serialize = "MagicSchool"))]
    pub school: &'a str,
    #[serde(rename(serialize = "Tier"))]
    pub level: i32,
}

impl<'a> From<&'a Model> for JsonCompatibleModel<'a> {
    fn from(value: &'a Model) -> Self {
        JsonCompatibleModel {
            id: value.id,
            school: value.school.to_json_compatible_repr(),
            level: value.level,
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

        let mut script_file = String::from("MCCS_SPELL_GENERATED_TABLE = {\n");
        for model in &self.entities {
            script_file += &model.to_lua_string();
        }
        script_file.push('}');
        zip_writer.start_file("scripts/generated/spells.lua", FileOptions::default())?;
        zip_writer.write_all(script_file.as_bytes())?;

        let _json_models = self
            .entities
            .iter()
            .filter_map(|m| {
                if m.school != MagicSchoolType::None {
                    Some(JsonCompatibleModel::from(m))
                } else {
                    None
                }
            })
            .collect_vec();

        // let json_string = serde_json::to_string_pretty(&json_models)?;
        // let mut file = std::fs::File::create("D:\\spells.json")?;
        // file.write_all(json_string.as_bytes())?;

        let mut json_file = std::fs::File::create("D:\\spells.json")?;
        let json_string = serde_json::to_string_pretty(&self.entities)?;
        json_file.write_all(json_string.as_bytes())?;

        Ok(())
    }
}
