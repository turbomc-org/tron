use crate::bridge::player_join_request::Edition as GrpcEdition;
use crate::models::achievements::Achievements;
use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::GENERATOR;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    #[serde(rename = "_id")]
    pub id: u64,
    pub username: String,
    pub discord_id: Option<String>,
    pub edition: Edition,
    pub coins: u64,
    pub prefixes: HashSet<String>,
    pub selected_prefix: Option<u64>,
    pub team: Option<u64>,
    pub friends: HashSet<u64>,
    pub invite_blocked: bool,
    pub kills: u64,
    pub rank: Rank,
    pub deaths: u64,
    pub blocks_broken: u64,
    pub blocks_placed: u64,
    pub vault_count: u64,
    pub owned_vault_ids: HashSet<String>,
    pub achievements: HashSet<Achievements>,
    pub redeemed_codes: HashSet<String>,
    #[serde(
        serialize_with = "crate::utils::serde::serialize_u64_map",
        deserialize_with = "crate::utils::serde::deserialize_u64_map"
    )]
    pub incoming_friend_requests: HashMap<u64, u64>,
    #[serde(
        serialize_with = "crate::utils::serde::serialize_u64_map",
        deserialize_with = "crate::utils::serde::deserialize_u64_map"
    )]
    pub incoming_team_requests: HashMap<u64, u64>,
    pub banned: u64,
    pub timed_out: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, Clone)]
pub enum Rank {
    Newbie,
    Member,
    Vip,
    Elite,
    Legend,
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Edition {
    Java,
    Bedrock,
}

impl From<GrpcEdition> for Edition {
    fn from(value: GrpcEdition) -> Self {
        match value {
            GrpcEdition::Java => Edition::Java,
            GrpcEdition::Bedrock => Edition::Bedrock,
        }
    }
}

impl Player {
    pub fn new(username: String, edition: Edition) -> Self {
        let now: DateTime<Utc> = Utc::now();

        Self {
            id: GENERATOR.generate(),
            username,
            edition,
            discord_id: None,
            coins: 0,
            kills: 0,
            deaths: 0,
            blocks_broken: 0,
            blocks_placed: 0,
            prefixes: HashSet::new(),
            selected_prefix: None,
            team: None,
            friends: HashSet::new(),
            invite_blocked: false,
            vault_count: 0,
            rank: Rank::Newbie,
            achievements: HashSet::new(),
            owned_vault_ids: HashSet::new(),
            redeemed_codes: HashSet::new(),
            incoming_friend_requests: HashMap::new(),
            incoming_team_requests: HashMap::new(),
            banned: 0,
            timed_out: 0,
            created_at: now.timestamp() as u64,
            updated_at: now.timestamp() as u64,
        }
    }
}
