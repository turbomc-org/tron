use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    #[serde(rename = "_id")]
    pub id: u64,
    pub player_id: u64,
    pub reported_by_player_id: u64,
    pub reason: String,
    pub created_at: u64,
}
