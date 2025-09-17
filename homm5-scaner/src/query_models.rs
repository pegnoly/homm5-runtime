use crate::scaners::prelude::CreatureDBEntity;
use sea_orm::{DerivePartialModel, FromQueryResult};

#[derive(Debug, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "CreatureDBEntity")]
pub struct CreatureXdbModel {
    pub _xdb: Option<String>,
}
