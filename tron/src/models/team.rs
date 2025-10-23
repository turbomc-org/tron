use crate::cache::Cache;
use crate::models::player::Player;
use anyhow::Result;
use bincode::{Decode, Encode};
use chrono::Utc;
use dashmap::DashMap;
use mongodb::Collection;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tonic::Status;
use tracing::error;

use crate::GENERATOR;
use crate::RETRY_STRATEGY;

#[derive(Serialize, Deserialize, Encode, Decode, Clone)]
pub struct Team {
    #[serde(rename = "_id")]
    pub id: u64,
    pub name: String,
    pub members: HashMap<u64, u64>,
    pub leader: u64,
    pub open: bool,
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
            banned_players: HashMap::new(),
            color,
        }
    }

    pub async fn insert(
        &self,
        col: &Collection<Self>,
        cache: &Arc<DashMap<u64, Self>>,
    ) -> Result<()> {
        let team_col = col.clone();
        let team = self.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                let team = team.clone();
                team_col.insert_one(team).await.map_err(|e| {
                    error!("Retrying team update due to: {}", e);
                    e
                })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Team update permanently failed: {}", e);
            }
        });

        cache.insert(self.id, self.clone());

        Ok(())
    }

    pub async fn get_team(
        player: &Player,
        cache: &Arc<DashMap<u64, Team>>,
    ) -> Result<Self, Status> {
        let team_id = player.team.ok_or_else(|| {
            error!("Player {} isn't in any team", player.username);
            Status::not_found(format!("Player {} isn't in any team", player.username))
        })?;

        match cache.get(&team_id).map(|entry| entry.clone()) {
            Some(team) => Ok(team),
            None => Err(Status::not_found(format!("Team not found"))),
        }
    }

    pub async fn add_member(
        &mut self,
        player: &mut Player,
        now: u64,
        p_col: &Collection<Player>,
        t_col: &Collection<Team>,
        p_cache: &Arc<DashMap<String, Player>>,
        t_cache: &Arc<DashMap<u64, Team>>,
    ) -> Result<()> {
        let p_col = p_col.clone();
        let t_col = t_col.clone();
        let player_id = player.id;
        let team_id = self.id;

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                p_col
                    .update_one(
                        doc! { "_id": player_id as i64 },
                        doc! { "$set": { "team": team_id as i64 }},
                    )
                    .await
                    .map_err(|e| {
                        error!("Retrying player update due to: {}", e);
                        e
                    })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Player update permanently failed: {}", e);
            }
        });

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                t_col
                    .update_one(
                        doc! { "_id": team_id as i64 },
                        doc! { "$push": { "members": player_id as i64 }},
                    )
                    .await
                    .map_err(|e| {
                        error!("Retrying team update due to: {}", e);
                        e
                    })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Team update permanently failed: {}", e);
            }
        });

        self.members.insert(player.id.clone(), now);
        player.team = Some(self.id);

        t_cache.insert(self.id, self.clone());
        p_cache.insert(player.username.clone(), player.clone());

        Ok(())
    }

    pub async fn remove_member(
        &mut self,
        player: &mut Player,
        p_col: &Collection<Player>,
        t_col: &Collection<Team>,
        p_cache: &Arc<DashMap<String, Player>>,
        t_cache: &Arc<DashMap<u64, Team>>,
    ) -> Result<()> {
        let p_col = p_col.clone();
        let t_col = t_col.clone();
        let player_id = player.id;
        let team_id = self.id;

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                p_col
                    .update_one(
                        doc! { "_id": player_id as i64 },
                        doc! { "$unset": { "team": "" }},
                    )
                    .await
                    .map_err(|e| {
                        error!("Retrying player update due to: {}", e);
                        e
                    })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Player update permanently failed: {}", e);
            }
        });

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                t_col
                    .update_one(
                        doc! { "_id": team_id as i64 },
                        doc! { "$pull": { "members": player_id as i64 }},
                    )
                    .await
                    .map_err(|e| {
                        error!("Retrying team update due to: {}", e);
                        e
                    })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Team update permanently failed: {}", e);
            }
        });

        self.members.remove(&player.id);
        player.team = None;

        t_cache.insert(self.id, self.clone());
        p_cache.insert(player.username.clone(), player.clone());

        Ok(())
    }

    pub async fn promote_player(
        &mut self,
        id: u64,
        col: &Collection<Self>,
        cache: &Arc<DashMap<u64, Team>>,
    ) -> Result<()> {
        let team_id = self.id.clone();
        let player_id = id.clone();
        let team_col = col.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                team_col
                    .update_one(
                        doc! { "_id": team_id as i64 },
                        doc! { "$set": { "leader": player_id as i64 }},
                    )
                    .await
                    .map_err(|e| {
                        error!("Retrying team update due to: {}", e);
                        e
                    })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Team update permanently failed: {}", e);
            }
        });

        self.leader = id;

        cache.insert(id.clone(), self.clone());

        Ok(())
    }

    pub async fn set_open(
        &mut self,
        open: bool,
        col: &Collection<Self>,
        cache: &Arc<DashMap<u64, Self>>,
    ) -> Result<()> {
        let team_col = col.clone();
        let team_id = self.id.clone();
        let value = open.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                team_col
                    .update_one(
                        doc! { "_id": team_id as i64 },
                        doc! { "$set": { "open": value}},
                    )
                    .await
                    .map_err(|e| {
                        error!("Retrying team update due to: {}", e);
                        e
                    })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Team update permanently failed: {}", e);
            }
        });

        self.open = open;

        cache.insert(self.id, self.clone());

        Ok(())
    }
}
