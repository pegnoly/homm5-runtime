use sea_orm::{sqlx::SqlitePool, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, SqlxSqlitePoolConnection};

use crate::{error::EditorToolsError, services::reserve_hero_creator::{model::{self, BaseSkill, Model, ReserveHeroSkills, ReserveHeroSpells}, payloads::InitReserveHeroPayload}};

pub struct ReserveHeroCreatorRepo {
    db: DatabaseConnection,
}

impl ReserveHeroCreatorRepo {
    pub fn new(pool: SqlitePool) -> Self {
        ReserveHeroCreatorRepo {
            db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)),
        }
    }

    pub async fn load_heroes(&self, map_id: i32, player_id: i32) -> Result<Vec<Model>, EditorToolsError> {
        let models = model::Entity::find()
                .filter(model::Column::MapId.eq(map_id))
                .filter(model::Column::PlayerId.eq(player_id))
                .all(&self.db).await?;
        Ok(models)
    }

    pub async fn load_existing_hero(&self, id: i32) -> Result<Model, EditorToolsError> {
        if let Some(existing_model) = model::Entity::find_by_id(id).one(&self.db).await? {
            return Ok(existing_model);
        }
        Err(EditorToolsError::SeaOrm(sea_orm::DbErr::RecordNotFound("No reserved hero matches given id".to_string())))
    }

    pub async fn init_hero(&self, payload: InitReserveHeroPayload) -> Result<Model, EditorToolsError> {
        let model_to_insert = model::ActiveModel {
            id: Default::default(),
            map_id: Set(payload.map_id),
            player_id: Set(payload.player_id),
            xdb_path: Set(payload.xdb),
            name: Set(payload.name),
            skills: Set(ReserveHeroSkills { skills: vec![BaseSkill { 
                slot: 0, 
                skill: String::from("HERO_SKILL_NONE"), 
                mastery: homm5_scaner::prelude::Mastery::MasteryNone,
                perks: vec![]
            }] }),
            spells: Set(ReserveHeroSpells { spells: vec![] })
        };
        Ok(model_to_insert.insert(&self.db).await?)
    }

    pub async fn add_skill(&self, id: i32, slot: i32) -> Result<BaseSkill, EditorToolsError> {
        if let Some(hero) = model::Entity::find_by_id(id).one(&self.db).await? {
            let mut model_to_update = hero.into_active_model();
            if let Some(mut skills) = model_to_update.skills.take() {
                let new_skill = BaseSkill { slot, skill: String::from("HERO_SKILL_NONE"), mastery: homm5_scaner::prelude::Mastery::MasteryNone, perks: vec![] };
                skills.skills.push(new_skill.clone());
                model_to_update.skills = Set(skills);
                model_to_update.update(&self.db).await?;
                return Ok(new_skill);
            }
            return Err(EditorToolsError::Default);
        }
        Err(EditorToolsError::SeaOrm(sea_orm::DbErr::RecordNotFound("No reserved hero matches given id".to_string())))
    }

    pub async fn remove_skill(&self, id: i32, slot: i32) -> Result<(), EditorToolsError> {
        if let Some(hero) = model::Entity::find_by_id(id).one(&self.db).await? {
            let mut model_to_update = hero.into_active_model();
            if let Some(mut skills) = model_to_update.skills.take() {
                skills.skills.retain(|s| s.slot != slot);
                model_to_update.skills = Set(skills);
                model_to_update.update(&self.db).await?;
                return Ok(());
            }
            return Err(EditorToolsError::Default);
        }
        Err(EditorToolsError::SeaOrm(sea_orm::DbErr::RecordNotFound("No reserved hero matches given id".to_string())))
    }

    pub async fn update_skill(&self, id: i32, slot: i32, skill: BaseSkill) -> Result<(), EditorToolsError> {
        if let Some(hero) = model::Entity::find_by_id(id).one(&self.db).await? {
            let mut model_to_update = hero.into_active_model();
            if let Some(mut skills) = model_to_update.skills.take() {
                if let Some(skill_to_update) = skills.skills.iter_mut().find(|s| s.slot == slot) {
                    *skill_to_update = skill;
                }
                model_to_update.skills = Set(skills);
            }
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn add_spell(&self, id: i32, spell: String) -> Result<(), EditorToolsError> {
        if let Some(hero) = model::Entity::find_by_id(id).one(&self.db).await? {
            let mut model_to_update = hero.into_active_model();
            if let Some(mut spells) = model_to_update.spells.take() {
                spells.spells.push(spell);
                model_to_update.spells = Set( spells );
                model_to_update.update(&self.db).await?;
                return Ok(())
            }
            return Err(EditorToolsError::Default);
        }
        Err(EditorToolsError::SeaOrm(sea_orm::DbErr::RecordNotFound("No reserved hero matches given id".to_string())))
    }

    pub async fn remove_spell(&self, id: i32, spell: String) -> Result<(), EditorToolsError> {
        if let Some(hero) = model::Entity::find_by_id(id).one(&self.db).await? {
            let mut model_to_update = hero.into_active_model();
            if let Some(mut spells) = model_to_update.spells.take() {
                spells.spells.retain(|s| *s != spell);
                model_to_update.spells = Set( spells );
                model_to_update.update(&self.db).await?;
                return Ok(())
            }
            return Err(EditorToolsError::Default);
        }
        Err(EditorToolsError::SeaOrm(sea_orm::DbErr::RecordNotFound("No reserved hero matches given id".to_string()))) 
    }

}