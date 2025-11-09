use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::GENERATOR;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bug {
    #[serde(rename = "_id")]
    pub id: u64,
    pub player_id: u64,
    pub description: String,
    pub created_at: u64,
}

impl Bug {
    pub fn new(player_id: u64, description: String) -> Self {
        Self {
            id: GENERATOR.generate(),
            player_id,
            description,
            created_at: Utc::now().timestamp() as u64,
        }
    }
}
