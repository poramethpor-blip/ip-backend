use crate::domain::value_objects::mission_brawler_model::MissionBrawlerModel;


#[derive(Debug, Clone)]
pub struct BrawlerViewEntity {
    pub id: i32,
    pub username: String,
}

impl BrawlerViewEntity {
    pub fn to_model(&self) -> MissionBrawlerModel {
        MissionBrawlerModel {
            id: self.id,
            username: self.username.clone(),
        }
    }
}
