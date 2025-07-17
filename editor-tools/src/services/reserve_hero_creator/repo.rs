use sea_orm::{sqlx::SqlitePool, ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel, SqlxSqlitePoolConnection};

use crate::{error::EditorToolsError, services::reserve_hero_creator::{model::{self, BaseSkill, Model, ReserveHeroPerks, ReserveHeroSkills, ReserveHeroSpells}, payloads::InitReserveHeroPayload}};

pub struct ReserveHeroCreatorRepo {
    db: DatabaseConnection,
}

impl ReserveHeroCreatorRepo {
    pub fn new(pool: SqlitePool) -> Self {
        ReserveHeroCreatorRepo {
            db: DatabaseConnection::SqlxSqlitePoolConnection(SqlxSqlitePoolConnection::from(pool)),
        }
    }

    pub async fn init_hero(&self, payload: InitReserveHeroPayload) -> Result<Model, EditorToolsError> {
        let model_to_insert = model::ActiveModel {
            id: Default::default(),
            xdb_path: Set(payload.xdb),
            name: Set(payload.name),
            primary_skill: Set(BaseSkill { skill: String::from("HERO_SKILL_NONE"), mastery: homm5_scaner::prelude::Mastery::MasteryNone}),
            skills: Set(ReserveHeroSkills { skills: vec![] }),
            perks: Set(ReserveHeroPerks { perks: vec![] }),
            spells: Set(ReserveHeroSpells { spells: vec![] })
        };
        Ok(model_to_insert.insert(&self.db).await?)
    }

    pub async fn update_primary_skill(&self, id: i32, skill: BaseSkill) -> Result<(), EditorToolsError> {
        if let Some(hero) = model::Entity::find_by_id(id).one(&self.db).await? {
            let mut model_to_update = hero.into_active_model();
            model_to_update.primary_skill = Set(skill);
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_skills(&self, id: i32, skills: Vec<BaseSkill>) -> Result<(), EditorToolsError> {
        if let Some(hero) = model::Entity::find_by_id(id).one(&self.db).await? {
            let mut model_to_update = hero.into_active_model();
            model_to_update.skills = Set(ReserveHeroSkills { skills });
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_perks(&self, id: i32, perks: Vec<String>) -> Result<(), EditorToolsError> {
        if let Some(hero) = model::Entity::find_by_id(id).one(&self.db).await? {
            let mut model_to_update = hero.into_active_model();
            model_to_update.perks = Set(ReserveHeroPerks { perks });
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn update_spells(&self, id: i32, spells: Vec<String>) -> Result<(), EditorToolsError> {
        if let Some(hero) = model::Entity::find_by_id(id).one(&self.db).await? {
            let mut model_to_update = hero.into_active_model();
            model_to_update.spells = Set(ReserveHeroSpells { spells });
            model_to_update.update(&self.db).await?;
        }
        Ok(())
    }
}