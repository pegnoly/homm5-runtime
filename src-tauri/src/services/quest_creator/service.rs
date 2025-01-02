use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use super::data::{QuestDBModel, QuestProgressDBModel};

pub struct QuestService {
    db_pool: Pool<Sqlite>
}

impl QuestService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        QuestService {
            db_pool: pool
        }
    }

    pub async fn get_quests_by_mission_data(&self, campaign_number: u32, mission_number: u32) -> Result<Vec<QuestDBModel>, sqlx::Error> {
        let quests: Result<Vec<QuestDBModel>, sqlx::Error> = sqlx::query_as(r#"
                SELECT * FROM quests WHERE campaign_number=? and mission_number=?
            "#)
            .bind(campaign_number)
            .bind(mission_number)
            .fetch_all(&self.db_pool)
            .await;

        quests
    }

    pub async fn get_quest_name(&self, quest_id: Uuid) -> Result<String, sqlx::Error> {
        let name: (String, ) = sqlx::query_as(r#"
                SELECT name FROM quests WHERE id=?;
            "#)
            .bind(quest_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(name.0)
    }

    pub async fn get_quest_desc(&self, quest_id: Uuid) -> Result<String, sqlx::Error> {
        let desc: (String, ) = sqlx::query_as(r#"
                SELECT desc FROM quests WHERE id=?;
            "#)
            .bind(quest_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(desc.0)
    }

    pub async fn get_quest_script_name(&self, quest_id: Uuid) -> Result<String, sqlx::Error> {
        let script_name: (String, ) = sqlx::query_as(r#"
                SELECT script_name FROM quests WHERE id=?;
            "#)
            .bind(quest_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(script_name.0)
    }

    pub async fn get_quest_directory(&self, quest_id: Uuid) -> Result<String, sqlx::Error> {
        let script_name: (String, ) = sqlx::query_as(r#"
                SELECT directory FROM quests WHERE id=?;
            "#)
            .bind(quest_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(script_name.0)
    }

    pub async fn is_secondary_quest(&self, quest_id: Uuid) -> Result<bool, sqlx::Error> {
        let is_secondary: (bool, ) = sqlx::query_as(r#"
                SELECT is_secondary FROM quests WHERE id=?;
            "#)
            .bind(quest_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(is_secondary.0)
    }

    pub async fn is_active_quest(&self, quest_id: Uuid) -> Result<bool, sqlx::Error> {
        let is_active: (bool, ) = sqlx::query_as(r#"
                SELECT is_active FROM quests WHERE id=?;
            "#)
            .bind(quest_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(is_active.0)
    }

    pub async fn create_quest(&self, directory: &String, script_name: &String, name: &String, campaign_number: u32, mission_number: u32) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();

        let _quest_insert_result = sqlx::query(r#"
                INSERT INTO quests (id, directory, campaign_number, mission_number, name, desc, script_name) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#)
                .bind(id)
                .bind(directory)
                .bind(campaign_number)
                .bind(mission_number)
                .bind(name)
                .bind(String::new())
                .bind(script_name)
                .execute(&self.db_pool)
                .await?;

        Ok(id)
    }

    pub async fn update_quest_directory(&self, quest_id: Uuid, new_directory: &String) -> Result<(), sqlx::Error> {
        let _res: QuestDBModel = sqlx::query_as(r#"
                UPDATE quests 
                SET directory=?
                WHERE id=?
                RETURNING *;
            "#)
                .bind(new_directory)
                .bind(quest_id)
                .fetch_one(&self.db_pool)
                .await?;

        Ok(())
    }

    pub async fn update_quest_script_name(&self, quest_id: Uuid, new_script_name: &String) -> Result<(), sqlx::Error> {
        let _res: QuestDBModel = sqlx::query_as(r#"
                UPDATE quests 
                SET script_name=?
                WHERE id=?
                RETURNING *;
            "#)
                .bind(new_script_name)
                .bind(quest_id)
                .fetch_one(&self.db_pool)
                .await?;

        Ok(())
    }

    pub async fn update_quest_name(&self, quest_id: Uuid, new_name: &String) -> Result<(), sqlx::Error> {
        let _res: QuestDBModel = sqlx::query_as(r#"
                UPDATE quests 
                SET name=?
                WHERE id=?
                RETURNING *;
            "#)
                .bind(new_name)
                .bind(quest_id)
                .fetch_one(&self.db_pool)
                .await?;

        Ok(())
    }

    pub async fn update_quest_desc(&self, quest_id: Uuid, new_desc: &String) -> Result<(), sqlx::Error> {
        let _res: QuestDBModel = sqlx::query_as(r#"
                UPDATE quests 
                SET desc=?
                WHERE id=?
                RETURNING *;
            "#)
                .bind(new_desc)
                .bind(quest_id)
                .fetch_one(&self.db_pool)
                .await?;

        Ok(())
    }

    pub async fn update_quest_is_active(&self, quest_id: Uuid, new_is_active: bool) -> Result<(), sqlx::Error> {
        let _res: QuestDBModel = sqlx::query_as(r#"
                UPDATE quests 
                SET is_active=?
                WHERE id=?
                RETURNING *;
            "#)
                .bind(new_is_active)
                .bind(quest_id)
                .fetch_one(&self.db_pool)
                .await?;

        Ok(())
    }

    pub async fn update_quest_is_secondary(&self, quest_id: Uuid, new_is_secondary: bool) -> Result<(), sqlx::Error> {
        let _res: QuestDBModel = sqlx::query_as(r#"
                UPDATE quests 
                SET is_secondary=?
                WHERE id=?
                RETURNING *;
            "#)
                .bind(new_is_secondary)
                .bind(quest_id)
                .fetch_one(&self.db_pool)
                .await?;

        Ok(())
    }

    pub async fn get_quest_progress(&self, quest_id: Uuid, progress_number: u32) -> Result<QuestProgressDBModel, sqlx::Error> {
        let existing_progress: Option<QuestProgressDBModel> = sqlx::query_as(r#"
                SELECT * FROM progresses WHERE quest_id=? AND number=?
            "#)                
            .bind(quest_id)
            .bind(progress_number)
            .fetch_optional(&self.db_pool)
            .await?;

        if let Some(progress) = existing_progress {
            return Ok(progress);
        }

        let progress_id = Uuid::new_v4();
        let res = sqlx::query_as(r#"
                INSERT INTO progresses (id, quest_id, number, text) VALUES (?, ?, ?, ?) RETURNING *;
            "#)
            .bind(progress_id)
            .bind(quest_id)
            .bind(progress_number)
            .bind(String::new())
            .fetch_one(&self.db_pool)
            .await?;

        Ok(res)
    }

    pub async fn save_progress(&self, quest_id: Uuid, progress_number: u32, text: &String) -> Result<(), sqlx::Error> {
        let _progress_update_result: QuestProgressDBModel = sqlx::query_as(r#"
                UPDATE progresses 
                SET text=?
                WHERE quest_id=? AND number=?
                RETURNING *;
            "#)
            .bind(text)
            .bind(quest_id)
            .bind(progress_number)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(())
    }

    pub async fn update_progress_concatenation(&self, quest_id: Uuid, progress_number: u32, concatenate: bool) -> Result<(), sqlx::Error> {
        let _update_result: QuestProgressDBModel = sqlx::query_as(r#"
                UPDATE progresses
                SET concatenate=?
                WHERE quest_id=? AND number=?
                RETURNING *;
            "#)
            .bind(concatenate)
            .bind(quest_id)
            .bind(progress_number)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(())
    }

    pub async fn add_quest_to_queue(&self, quest_id: Uuid, map_id: u16) -> Result<(), sqlx::Error> {
        let _quest_add_result = sqlx::query(r#"
                INSERT INTO quest_modifiers (quest_id, map_id) VALUES (?, ?)
                ON CONFLICT
                DO NOTHING;
            "#)
            .bind(quest_id)
            .bind(map_id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    pub async fn get_quests_to_apply(&self, map_id: u16) -> Result<Vec<QuestDBModel>, sqlx::Error> {
        let quests: Vec<QuestDBModel> = sqlx::query_as(r#"
                SELECT * FROM quests 
                WHERE id IN
                (SELECT quest_id FROM quest_modifiers WHERE map_id=?)
            "#)
            .bind(map_id)
            .fetch_all(&self.db_pool)
            .await?;

        Ok(quests)
    }

    pub async fn get_quest_progresses(&self, quest_id: Uuid) -> Result<Vec<QuestProgressDBModel>, sqlx::Error> {
        let progresses: Vec<QuestProgressDBModel> = sqlx::query_as(r#"
                SELECT * FROM progresses WHERE quest_id=?
            "#)
            .bind(quest_id)
            .fetch_all(&self.db_pool)
            .await?;

        Ok(progresses)
    }

    pub async fn delete_quests_from_queue(&self, map_id: u16) -> Result<(), sqlx::Error> {
        let _queue_deletion_result = sqlx::query(r#"
                DELETE FROM quest_modifiers WHERE map_id=?
            "#)
            .bind(map_id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }
}