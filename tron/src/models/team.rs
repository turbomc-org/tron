use crate::cache::Cache;
use crate::models::player::Player;
use anyhow::Result;
use bincode::{Decode, Encode};
use chrono::Utc;
use dashmap::DashMap;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tonic::Status;
use tracing::error;

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

    pub async fn insert(
        &self,
        col: &Collection<Self>,
        cache: &Arc<DashMap<u64, Self>>,
    ) -> Result<()> {
        col.insert_one(self).await?;
        cache.insert(self.id, self.clone());

        Ok(())
    }

    pub async fn get_team(player: &Player, cache: &Cache) -> Result<Self, Status> {
        let team_id = player.team.ok_or_else(|| {
            error!("Player {} isn't in any team", player.username);
            Status::not_found(format!("Player {} isn't in any team", player.username))
        })?;

        let team = cache.get_team(team_id).await?;

        Ok(team)
    }
}
