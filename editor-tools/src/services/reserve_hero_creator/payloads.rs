use homm5_scaner::prelude::Town;

pub struct InitReserveHeroPayload {
    pub map_id: i32,
    pub player_id: i32,
    pub name: String,
    pub xdb: String,
    pub town: Town,
}
