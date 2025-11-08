use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bug {
    #[serde(rename = "_id")]
    pub id: u64,
    pub player_id: u64,
    pub description: String,
    pub created_at: u64,
}
