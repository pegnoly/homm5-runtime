use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use super::data::{
    DialogDBModel, DialogFrontendModel, DialogVariantModel, SpeakerDBModel, SpeakerFrontendModel,
    SpeakerType,
};

pub struct DialogGeneratorService {
    db_pool: Pool<Sqlite>,
}

impl DialogGeneratorService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        DialogGeneratorService { db_pool: pool }
    }

    pub async fn get_dialog(&self, id: Uuid) -> Result<DialogDBModel, sqlx::Error> {
        Ok(sqlx::query_as(
            r#"
                SELECT * FROM dialogs WHERE id=?;
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?)
    }

    pub async fn get_dialogs(&self) -> Result<Vec<DialogFrontendModel>, sqlx::Error> {
        let dialogs: Vec<DialogDBModel> = sqlx::query_as(
            r#"
                SELECT * FROM dialogs;
            "#,
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(dialogs
            .into_iter()
            .map(|d| DialogFrontendModel::from(d))
            .collect())
    }

    pub async fn get_speakers(&self) -> Result<Vec<SpeakerFrontendModel>, sqlx::Error> {
        let speakers: Vec<SpeakerDBModel> = sqlx::query_as(
            r#"
                SELECT * FROM speakers;
            "#,
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(speakers
            .into_iter()
            .map(|sp| SpeakerFrontendModel::from(sp))
            .collect())
    }

    pub async fn get_speakers_by_ids(
        &self,
        ids: &Vec<Uuid>,
    ) -> Result<Vec<SpeakerDBModel>, sqlx::Error> {
        let ids_params = format!("?{}", ", ?".repeat(ids.len() - 1));
        let sql = format!("SELECT * FROM speakers WHERE id IN ({})", ids_params);
        let mut query = sqlx::query_as(&sql);
        for id in ids {
            query = query.bind(id);
        }

        let res: Vec<SpeakerDBModel> = query.fetch_all(&self.db_pool).await?;

        Ok(res)
    }

    pub async fn get_variants_for_dialog(
        &self,
        id: Uuid,
    ) -> Result<Vec<DialogVariantModel>, sqlx::Error> {
        let variants: Vec<DialogVariantModel> = sqlx::query_as(
            r#"
                SELECT * FROM variants WHERE dialog_id=?;
            "#,
        )
        .bind(id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(variants)
    }

    pub async fn create_dialog(
        &self,
        name: &String,
        script_name: &String,
        directory: &String,
        speakers: &Vec<String>,
    ) -> Result<DialogFrontendModel, sqlx::Error> {
        let id = Uuid::new_v4();
        let _res = sqlx::query(
            r#"
                INSERT INTO dialogs 
                (id, name, script_name, directory, speakers_ids, labels)
                VALUES (?, ?, ?, ?, ?, ?);
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(script_name)
        .bind(directory)
        .bind(serde_json::to_string(speakers).unwrap())
        .bind(serde_json::to_string(&vec!["main".to_string()]).unwrap())
        .execute(&self.db_pool)
        .await?;

        Ok(DialogFrontendModel {
            id: id,
            name: name.clone(),
        })
    }

    pub async fn get_dialog_name(&self, id: Uuid) -> Result<String, sqlx::Error> {
        let name: (String,) = sqlx::query_as(
            r#"
                SELECT name FROM dialogs WHERE id=?;
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(name.0)
    }

    pub async fn get_dialog_script_name(&self, id: Uuid) -> Result<String, sqlx::Error> {
        let script_name: (String,) = sqlx::query_as(
            r#"
                SELECT script_name FROM dialogs WHERE id=?;
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(script_name.0)
    }

    pub async fn get_dialog_directory(&self, id: Uuid) -> Result<String, sqlx::Error> {
        let directory: (String,) = sqlx::query_as(
            r#"
                SELECT directory FROM dialogs WHERE id=?;
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(directory.0)
    }

    pub async fn get_dialog_speakers(&self, id: Uuid) -> Result<Vec<Uuid>, super::error::Error> {
        let speakers: (String,) = sqlx::query_as(
            r#"
                SELECT speakers_ids FROM dialogs WHERE id=?;
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        let speakers_ids: Vec<Uuid> = serde_json::from_str(&speakers.0)?;

        Ok(speakers_ids)
    }

    pub async fn get_dialog_labels(&self, id: Uuid) -> Result<Vec<String>, super::error::Error> {
        let labels: (String,) = sqlx::query_as(
            r#"
                SELECT labels FROM dialogs WHERE id=?;
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        let labels: Vec<String> = serde_json::from_str(&labels.0)?;

        Ok(labels)
    }

    pub async fn update_dialog_labels(
        &self,
        id: Uuid,
        labels: &Vec<String>,
    ) -> Result<(), super::error::Error> {
        let updated_labels = serde_json::to_string(labels)?;

        let _res: DialogDBModel = sqlx::query_as(
            r#"
                UPDATE dialogs
                SET labels=?
                WHERE id=?
                RETURNING *;
            "#,
        )
        .bind(updated_labels)
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn set_dialog_was_generated(
        &self,
        id: Uuid,
        was_generated: bool,
    ) -> Result<(), super::error::Error> {
        let _res: DialogDBModel = sqlx::query_as(
            r#"
            UPDATE dialogs
            SET was_generated=?
            WHERE id=?
            RETURNING *;
        "#,
        )
        .bind(was_generated)
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn get_dialog_variant_id(
        &self,
        dialog_id: Uuid,
        dialog_step: u32,
        dialog_label: &String,
    ) -> Result<Uuid, sqlx::Error> {
        let existing_variant: Option<(Uuid,)> = sqlx::query_as(
            r#"
                SELECT id FROM variants WHERE dialog_id=? AND step=? AND label=?;
            "#,
        )
        .bind(dialog_id)
        .bind(dialog_step)
        .bind(dialog_label)
        .fetch_optional(&self.db_pool)
        .await?;

        match existing_variant {
            Some(variant) => Ok(variant.0),
            None => {
                let new_variant_id = self
                    .create_new_dialog_variant(dialog_id, dialog_step, dialog_label)
                    .await?;
                Ok(new_variant_id)
            }
        }
    }

    pub async fn create_speaker(
        &self,
        name: &String,
        script_name: &String,
        color: &String,
        speaker_type: SpeakerType,
    ) -> Result<SpeakerFrontendModel, sqlx::Error> {
        let id = Uuid::new_v4();
        let _res = sqlx::query(
            r#"
                INSERT INTO speakers 
                (id, name, script_name, color, speaker_type)
                VALUES (?, ?, ?, ?, ?);
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(script_name)
        .bind(color)
        .bind(speaker_type)
        .execute(&self.db_pool)
        .await?;

        Ok(SpeakerFrontendModel {
            id: id,
            name: name.clone(),
        })
    }

    pub async fn create_new_dialog_variant(
        &self,
        dialog_id: Uuid,
        dialog_step: u32,
        dialog_label: &String,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let _res = sqlx::query(
            r#"
                INSERT INTO variants (id, dialog_id, step, label, speaker_id, text)
                VALUES(?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(dialog_id)
        .bind(dialog_step)
        .bind(dialog_label)
        .bind::<Option<Uuid>>(None)
        .bind(String::new())
        .execute(&self.db_pool)
        .await?;

        Ok(id)
    }

    pub async fn get_dialog_variant_speaker_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let speaker: (Option<Uuid>,) = sqlx::query_as(
            r#"
                SELECT speaker_id FROM variants WHERE id=?
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(speaker.0)
    }

    pub async fn get_dialog_variant_text(&self, id: Uuid) -> Result<String, sqlx::Error> {
        let text: (String,) = sqlx::query_as(
            r#"
                SELECT text FROM variants WHERE id=?
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(text.0)
    }

    pub async fn save_dialog_variant(
        &self,
        id: Uuid,
        speaker_id: Uuid,
        text: &String,
    ) -> Result<(), sqlx::Error> {
        let _res = sqlx::query(
            r#"
                UPDATE variants 
                SET speaker_id=?, text=?
                WHERE id=?;
            "#,
        )
        .bind(speaker_id)
        .bind(text)
        .bind(id)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
