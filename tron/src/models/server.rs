use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::GENERATOR;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: u64,
    pub name: String,
    pub description: String,
    pub address: String,
    pub created_by: u64,
    pub created_at: u64,
}

impl Server {
    pub fn new(player_id: u64, name: String, description: String, address: String) -> Self {
        Self {
            id: GENERATOR.generate(),
            name,
            description,
            address,
            created_by: player_id,
            created_at: Utc::now().timestamp() as u64,
        }
    }
}
