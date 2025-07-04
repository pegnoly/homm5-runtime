use sea_orm::{DerivePartialModel, FromQueryResult};
use crate::scaners::prelude::CreatureDBEntity;

#[derive(Debug, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "CreatureDBEntity")]
pub struct CreatureXdbModel {
    pub xdb: Option<String>
}