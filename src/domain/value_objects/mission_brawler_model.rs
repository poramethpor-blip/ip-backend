use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionBrawlerModel {
    pub id: i32,
    pub username: String,
}
