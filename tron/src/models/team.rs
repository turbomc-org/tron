use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Team {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub members: Vec<String>,
    pub leader: String,
    pub open: bool,
    #[serde(rename = "requestBlocked")]
    pub request_blocked: bool,
    #[serde(rename = "bannedPlayers")]
    pub banned_players: Vec<String>,
    pub color: String,
}
