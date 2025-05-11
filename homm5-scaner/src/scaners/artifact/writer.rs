use sea_orm::{sea_query::OnConflict, DatabaseConnection, EntityTrait, IntoActiveModel, Iterable, TransactionTrait};

use crate::{core::Output, error::ScanerError};

use super::model::Column;

pub struct ArtifactDataOutput<'a> {
    entities: Vec<super::model::Model>,
    db: &'a DatabaseConnection
}

impl<'a> ArtifactDataOutput<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        ArtifactDataOutput { entities: vec![], db }
    }
}

impl<'a> Output for ArtifactDataOutput<'a> { 
    type Input = super::model::Model;

    fn output_single(&mut self, object: Self::Input) -> Result<(), ScanerError> {
        self.entities.push(object);
        Ok(())
    }

    async fn finish_output(&self) -> Result<(), ScanerError> {
        let transaction = self.db.begin().await?;
        let on_conflict = OnConflict::new()
            .update_columns(
                super::model::Column::iter()
                    .filter_map(|column| {
                        match column {
                            Column::Id => None,
                            _=> Some(column)
                        }
                    })
                    .collect::<Vec<super::model::Column>>()
            ).to_owned();
        for entity in &self.entities {
            let active_model = entity.clone().into_active_model();
            super::model::Entity::insert(active_model).on_conflict(on_conflict.clone()).exec(&transaction).await?;
        }
        transaction.commit().await?;
        Ok(())
    }
}