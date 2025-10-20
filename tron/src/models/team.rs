use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Team {
    #[serde(rename = "_id")]
    pub id: u64,
    pub name: String,
    pub display_name: String,
    pub members: Vec<String>,
    pub leader: String,
    pub open: bool,
    pub request_blocked: bool,
    pub banned_players: Vec<String>,
    pub color: String,
}
