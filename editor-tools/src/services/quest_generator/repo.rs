use sea_orm::{sqlx::SqlitePool, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, SqlxSqlitePoolConnection};

use crate::{error::EditorToolsError, services::quest_generator::{models::{progress, quest}, payloads::{CreateQuestPayload, GetProgressPayload, SaveProgressPayload, UpdateQuestPayload}, prelude::{QuestModel, QuestProgressModel}}};


pub struct QuestGeneratorRepo {
    db: DatabaseConnection,
}

impl QuestGeneratorRepo {
    pub fn new(pool: SqlitePool) -> Self {
        QuestGeneratorRepo {
            db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)),
        }
    }

    pub async fn load_quests(&self, mission_id: i32) -> Result<Vec<QuestModel>, EditorToolsError> {
        Ok(quest::Entity::find().filter(quest::Column::MissionId.eq(mission_id)).all(&self.db).await?)
    }

    pub async fn load_quest(&self, id: i32) -> Result<Option<QuestModel>, EditorToolsError> {
        Ok(quest::Entity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn create_quest(&self, payload: CreateQuestPayload) -> Result<QuestModel, EditorToolsError> {
        let model_to_insert = quest::ActiveModel {
            mission_id: Set(payload.mission_id),
            name: Set(payload.name),
            script_name: Set(payload.script_name),
            directory: Set(payload.directory),
            ..Default::default()
        };
        Ok(model_to_insert.insert(&self.db).await?)
    }

    pub async fn update_quest(&self, payload: UpdateQuestPayload) -> Result<(), EditorToolsError> {
        if let Some(existing_quest) = quest::Entity::find_by_id(payload.id).one(&self.db).await? {
            let mut quest_to_update = existing_quest.into_active_model();
            if let Some(name) = payload.name {
                quest_to_update.name = Set(name);
            }
            if let Some(desc) = payload.desc {
                quest_to_update.desc = Set(desc);
            }
            if let Some(directory) = payload.directory {
                quest_to_update.directory = Set(directory);
            }
            if let Some(secondary) = payload.is_secondary {
                quest_to_update.is_secondary = Set(secondary);
            }
            if let Some(active) = payload.is_active {
                quest_to_update.is_active = Set(active);
            }
            quest_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn get_progress(&self, payload: GetProgressPayload) -> Result<Option<QuestProgressModel>, EditorToolsError> {
        Ok(progress::Entity::find()
            .filter(progress::Column::QuestId.eq(payload.quest_id))
            .filter(progress::Column::Number.eq(payload.number))
            .one(&self.db)
            .await?
        )
    }

    pub async fn create_progress(&self, payload: GetProgressPayload) -> Result<QuestProgressModel, EditorToolsError> {
        let model_to_insert = progress::ActiveModel {
            number: Set(payload.number),
            quest_id: Set(payload.quest_id),
            ..Default::default()
        };
        Ok(model_to_insert.insert(&self.db).await?)
    }

    pub async fn save_progress(&self, payload: SaveProgressPayload) -> Result<(), EditorToolsError> {
        if let Some(existing_progress) = progress::Entity::find_by_id(payload.id).one(&self.db).await? {
            let mut progress_to_update = existing_progress.into_active_model();
            progress_to_update.concatenate = Set(payload.concatenate);
            progress_to_update.text = Set(payload.text);
            progress_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn load_progresses(&self, quest_id: i32) -> Result<Vec<QuestProgressModel>, EditorToolsError> {
        Ok(progress::Entity::find().filter(progress::Column::QuestId.eq(quest_id)).all(&self.db).await?)
    }
}