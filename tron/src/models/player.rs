use crate::models::achievements::Achievements;
use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tron_protos::Edition as GrpcEdition;
use tron_protos::Rank as ProtoRank;

use crate::GENERATOR;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    #[serde(rename = "_id")]
    pub id: u64,
    pub username: String,
    pub discord_id: Option<u64>,
    pub edition: Edition,
    pub role: Role,
    pub coins: u64,
    pub prefixes: HashSet<u64>,
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
    pub redeemed_codes: HashSet<u64>,
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
    pub scoreboard_enabled: bool,
    pub banned: u64,
    pub timed_out: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
#[serde(rename_all = "UPPERCASE")]
#[repr(i32)]
pub enum Rank {
    #[default]
    Newbie = 0,
    Member = 1,
    Vip = 2,
    Elite = 3,
    Legend = 4,
}

impl From<ProtoRank> for Rank {
    #[inline(always)]
    fn from(p: ProtoRank) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl From<Rank> for ProtoRank {
    #[inline(always)]
    fn from(a: Rank) -> Self {
        unsafe { std::mem::transmute(a) }
    }
}

impl TryFrom<i32> for Rank {
    type Error = ();

    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::Newbie),
            1 => Ok(Rank::Member),
            2 => Ok(Rank::Vip),
            3 => Ok(Rank::Elite),
            4 => Ok(Rank::Legend),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode, PartialEq, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum Edition {
    Java,
    Bedrock,
}

impl TryFrom<i32> for Edition {
    type Error = ();

    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Edition::Java),
            1 => Ok(Edition::Bedrock),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Role {
    Admin,
    Moderator,
    Member,
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
            role: Role::Member,
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
            scoreboard_enabled: true,
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

    pub fn compile_rank(&self) -> i32 {
        match self.rank {
            Rank::Newbie => 0,
            Rank::Member => 1,
            Rank::Vip => 2,
            Rank::Elite => 3,
            Rank::Legend => 4,
        }
    }
}
