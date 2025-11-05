use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::GENERATOR;

#[derive(Serialize, Deserialize)]
pub struct Server {
    #[serde(rename = "_id")]
    id: u64,
    name: String,
    description: String,
    address: String,
    port: u16,
    created_by: u64,
    created_at: u64,
}

impl Server {
    pub fn new(
        player_id: u64,
        name: String,
        description: String,
        address: String,
        port: u16,
    ) -> Self {
        Self {
            id: GENERATOR.generate(),
            name,
            description,
            address,
            port,
            created_by: player_id,
            created_at: Utc::now().timestamp() as u64,
        }
    }
}
