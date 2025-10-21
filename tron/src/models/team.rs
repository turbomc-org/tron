use bincode::{Decode, Encode};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::GENERATOR;

#[derive(Serialize, Deserialize, Encode, Decode, Clone)]
pub struct Team {
    #[serde(rename = "_id")]
    pub id: u64,
    pub name: String,
    pub members: HashMap<u64, u64>,
    pub leader: u64,
    pub open: bool,
    pub request_blocked: bool,
    pub banned_players: HashMap<u64, u64>,
    pub color: String,
}

impl Team {
    pub fn new(name: String, leader: u64, open: bool, color: String) -> Self {
        let now = Utc::now().timestamp() as u64;
        let mut members = HashMap::new();
        members.insert(leader.clone(), now);

        Self {
            id: GENERATOR.generate(),
            name,
            members,
            leader,
            open,
            request_blocked: false,
            banned_players: HashMap::new(),
            color,
        }
    }
}
