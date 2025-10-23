use crate::bridge::player_join_request::Edition as GrpcEdition;
use crate::models::team::Team;
use anyhow::Result;
use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error as MongoError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tracing::error;

use crate::GENERATOR;
use crate::RETRY_STRATEGY;

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
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
    pub vault_count: u64,
    pub owned_vault_ids: HashSet<String>,
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

impl Player {
    pub fn coins(&self) -> u64 {
        self.coins
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn kills(&self) -> u64 {
        self.kills
    }

    pub fn deaths(&self) -> u64 {
        self.deaths
    }
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

pub enum FindByDiscordErrors {
    NotFound,
    InternalError,
}

#[derive(Debug)]
pub enum IncCoinsError {
    PlayerNotFound,
    InsufficientCoins,
    MongoError(MongoError),
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
            prefixes: HashSet::new(),
            selected_prefix: None,
            team: None,
            friends: HashSet::new(),
            invite_blocked: false,
            vault_count: 0,
            rank: Rank::Newbie,
            owned_vault_ids: HashSet::new(),
            redeemed_codes: HashSet::new(),
            incoming_friend_requests: HashMap::new(),
            incoming_team_requests: HashMap::new(),
            created_at: now.timestamp() as u64,
            updated_at: now.timestamp() as u64,
        }
    }

    pub async fn insert(
        &self,
        player_col: &Collection<Self>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        player_col.insert_one(self.clone()).await?;

        cache.insert(self.username.clone(), self.clone());

        Ok(())
    }

    pub async fn find_by_username(
        username: String,
        col: &Collection<Self>,
    ) -> anyhow::Result<Option<Self>> {
        col.find_one(doc! {
            "username": username
        })
        .await
        .map_err(|_| anyhow::anyhow!("Failed to find player by username"))
    }

    pub async fn find_by_discord(
        id: String,
        col: Collection<Self>,
    ) -> Result<Self, FindByDiscordErrors> {
        match col
            .find_one(doc! {
                "discordId": id
            })
            .await
        {
            Ok(Some(player)) => Ok(player),
            Ok(None) => Err(FindByDiscordErrors::NotFound),
            Err(_) => Err(FindByDiscordErrors::InternalError),
        }
    }

    pub async fn inc_coins(id: u64, coins: i64, col: &Collection<Self>) -> Result<()> {
        col.update_one(
            doc! { "_id": id as i64 },
            doc! {
                "$inc": { "coins": coins },
            },
        )
        .await?;

        Ok(())
    }

    pub async fn transfer_coins(
        from_id: u64,
        to_id: u64,
        amount: u64,
        col: &Collection<Self>,
    ) -> anyhow::Result<()> {
        Self::inc_coins(from_id, -(amount as i64), col).await?;
        Self::inc_coins(to_id, amount as i64, col).await?;

        Ok(())
    }

    pub async fn add_friend_request(
        sender: u64,
        receiver: u64,
        now: u64,
        col: &Collection<Player>,
    ) -> anyhow::Result<()> {
        col.update_one(
            doc! { "_id": receiver as i64 },
            doc! {
                "$set": {
                    "incoming_friend_requests": {
                        (sender as i64).to_string(): now as i64
                    }
                }
            },
        )
        .await?;

        Ok(())
    }

    pub async fn accept_friend_request(
        id: u64,
        sender: u64,
        col: &Collection<Player>,
    ) -> anyhow::Result<()> {
        col.update_one(
            doc! { "_id": id.clone() as i64 },
            doc! {
                "$unset": { format!("incoming_friend_requests.{}", sender): "" },
                "$addToSet": { "friends": sender as i64}
            },
        )
        .await?;

        col.update_one(
            doc! {"_id": sender.clone() as i64},
            doc! {
                "$addToSet": { "friends": id as i64}
            },
        )
        .await?;

        Ok(())
    }

    pub async fn reject_friend_request(
        id: u64,
        sender: u64,
        col: &Collection<Player>,
    ) -> anyhow::Result<()> {
        col.update_one(
            doc! {"_id": id as i64},
            doc! {
                "$unset": { format!("incoming_friend_requests.{}", sender as i64): "" },
            },
        )
        .await?;

        Ok(())
    }

    pub async fn remove_friend(&self, target: u64, col: &Collection<Player>) -> anyhow::Result<()> {
        col.update_one(
            doc! {"_id": self.id as i64},
            doc! {
                "$pull": { "friends": target as i64 }
            },
        )
        .await?;

        Ok(())
    }

    pub async fn accept_team_request(
        &mut self,
        team_id: u64,
        now: u64,
        p_col: &Collection<Player>,
        t_col: &Collection<Team>,
        p_cache: &Arc<DashMap<String, Self>>,
        t_cache: &Arc<DashMap<u64, Team>>,
    ) -> Result<()> {
        let p_col = p_col.clone();
        let t_col = t_col.clone();
        let player_id = self.id.clone();
        let team_id = team_id.clone();

        task::spawn({
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    p_col.update_one(
                        doc! { "_id": player_id as i64 },
                        doc! {
                            "$unset": { format!("incoming_team_requests.{}", team_id as i64): "" },
                            "$set": { "team": team_id as i64}
                        },
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
            }
        });

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                t_col
                    .update_one(
                        doc! {"_id": team_id as i64},
                        doc! {
                            "$set": {
                                "members": {
                                    (player_id as i64).to_string(): now as i64
                                }
                            }
                        },
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

        self.incoming_team_requests.remove(&team_id);
        p_cache.insert(self.username.clone(), self.clone());
        let mut team: Team = t_cache.get(&team_id).unwrap().clone();
        team.members.insert(self.id.clone(), now.clone());

        Ok(())
    }

    pub async fn reject_team_request(
        &mut self,
        team_id: u64,
        col: &Collection<Player>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> anyhow::Result<()> {
        let player_id = self.id;
        let team_id = team_id.clone();
        let col = col.clone();

        task::spawn({
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.update_one(
                        doc! { "_id": player_id as i64 },
                        doc! {
                            "$unset": { format!("incoming_team_requests.{}", team_id as i64): "" },
                        },
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
            }
        });

        self.incoming_team_requests.remove(&team_id);
        cache.insert(self.username.clone(), self.clone());

        Ok(())
    }

    pub async fn add_team_invite(
        &mut self,
        team_id: u64,
        now: u64,
        col: &Collection<Player>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let player_id = self.id.clone();
        let team_id = team_id.clone();
        let col = col.clone();

        task::spawn({
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.update_one(
                        doc! { "_id": player_id as i64 },
                        doc! {
                            "$set": { format!("incoming_team_requests.{}", team_id as i64): now as i64 },
                        },
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
            }
        });

        self.incoming_team_requests
            .insert(team_id.clone(), now.clone());
        cache.insert(self.username.clone(), self.clone());

        Ok(())
    }
}
