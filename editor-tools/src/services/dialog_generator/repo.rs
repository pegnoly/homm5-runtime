use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, SqlxSqlitePoolConnection, sqlx::SqlitePool,
};

use crate::{
    error::EditorToolsError,
    prelude::{
        CreateDialogPayload, CreateDialogVariantPayload, CreateSpeakerPayload, DialogModel,
        DialogVariantModel, GetDialogVariantPayload, SaveVariantPayload, SpeakerModel,
        UpdateLabelsPayload,
    },
    services::dialog_generator::models::{
        dialog::{self, Labels, SpeakersIds},
        speaker, variant,
    },
};

pub struct DialogGeneratorRepo {
    db: DatabaseConnection,
}

impl DialogGeneratorRepo {
    pub fn new(pool: SqlitePool) -> Self {
        DialogGeneratorRepo {
            db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)),
        }
    }

    pub async fn load_dialogs(
        &self,
        mission_id: i32,
    ) -> Result<Vec<DialogModel>, EditorToolsError> {
        Ok(dialog::Entity::find()
            .filter(dialog::Column::MissionId.eq(mission_id))
            .all(&self.db)
            .await?)
    }

    pub async fn load_speakers(&self) -> Result<Vec<SpeakerModel>, EditorToolsError> {
        Ok(speaker::Entity::find().all(&self.db).await?)
    }

    pub async fn create_dialog(
        &self,
        payload: CreateDialogPayload,
    ) -> Result<DialogModel, EditorToolsError> {
        let model_to_insert = dialog::ActiveModel {
            mission_id: Set(payload.mission_id),
            name: Set(payload.name),
            script_name: Set(payload.script_name),
            directory: Set(payload.directory),
            labels: Set(Labels {
                labels: vec!["main".to_string()],
            }),
            speakers_ids: Set(SpeakersIds {
                ids: payload.speakers,
            }),
            was_generated: Set(false),
            ..Default::default()
        };
        Ok(model_to_insert.insert(&self.db).await?)
    }

    pub async fn create_speaker(
        &self,
        payload: CreateSpeakerPayload,
    ) -> Result<SpeakerModel, EditorToolsError> {
        let model_to_insert = speaker::ActiveModel {
            name: Set(payload.name),
            script_name: Set(payload.script_name),
            color: Set(payload.color),
            speaker_type: Set(payload.speaker_type),
            ..Default::default()
        };
        Ok(model_to_insert.insert(&self.db).await?)
    }

    pub async fn get_dialog(&self, id: i32) -> Result<Option<DialogModel>, EditorToolsError> {
        Ok(dialog::Entity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn get_variant(
        &self,
        payload: GetDialogVariantPayload,
    ) -> Result<Option<DialogVariantModel>, EditorToolsError> {
        let model = variant::Entity::find()
            .filter(variant::Column::DialogId.eq(payload.dialog_id))
            .filter(variant::Column::Step.eq(payload.step))
            .filter(variant::Column::Label.eq(payload.label))
            .one(&self.db)
            .await?;
        Ok(model)
    }

    pub async fn update_dialog_labels(
        &self,
        payload: UpdateLabelsPayload,
    ) -> Result<(), EditorToolsError> {
        if let Some(existing_dialog) = dialog::Entity::find_by_id(payload.dialog_id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_dialog.into_active_model();
            model_to_update.labels = Set(Labels {
                labels: payload.labels,
            });
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn create_variant(
        &self,
        payload: CreateDialogVariantPayload,
    ) -> Result<DialogVariantModel, EditorToolsError> {
        let model_to_insert = variant::ActiveModel {
            dialog_id: Set(payload.dialog_id),
            step: Set(payload.step),
            label: Set(payload.label),
            text: Set("".to_string()),
            ..Default::default()
        };
        Ok(model_to_insert.insert(&self.db).await?)
    }

    pub async fn save_variant(&self, payload: SaveVariantPayload) -> Result<(), EditorToolsError> {
        if let Some(existing_variant) = variant::Entity::find_by_id(payload.id)
            .one(&self.db)
            .await?
        {
            let mut model_to_update = existing_variant.into_active_model();
            model_to_update.text = Set(payload.text);
            model_to_update.speaker_id = Set(Some(payload.speaker));
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn get_speakers_by_ids(&self, ids: Vec<i32>) -> Result<Vec<SpeakerModel>, EditorToolsError> {
        Ok(speaker::Entity::find().filter(speaker::Column::Id.is_in(ids)).all(&self.db).await?)
    }

    pub async fn get_all_variants_for_dialog(&self, id: i32) -> Result<Vec<DialogVariantModel>, EditorToolsError> {
        Ok(variant::Entity::find().filter(variant::Column::DialogId.eq(id)).all(&self.db).await?)
    }
}
