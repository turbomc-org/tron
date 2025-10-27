use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: u64,
    pub player: u64,
    pub message: String,
    pub created_at: u64,
}
