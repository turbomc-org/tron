use crate::RETRY_STRATEGY;
use crate::collections::player::PlayerCollection;
use crate::collections::team::TeamCollection;
use crate::models::player::{Player, Rank};
use crate::models::team::Team;
use anyhow::Result;
use dashmap::DashMap;
use mongodb::Collection;
use mongodb::bson::doc;
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tracing::{debug, error};

impl Player {
    pub async fn insert(
        &self,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let player = self.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.insert_one(&player).await.map_err(|e| {
                        error!("Retrying player insertion due to: {}", e);
                        e
                    })
                })
                .await;

                if let Err(e) = retry_result {
                    error!("Player insertion permanently failed: {}", e);
                }
            }
        });

        cache.insert(self.username.clone(), self.clone());

        Ok(())
    }

    pub async fn find_by_username(
        username: &str,
        col: &Arc<dyn PlayerCollection>,
    ) -> anyhow::Result<Option<Self>> {
        col.find_by_username(username)
            .await
            .map_err(|_| anyhow::anyhow!("Failed to find player by username"))
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
        &mut self,
        target: &mut Self,
        amount: u64,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let player_id = self.id.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.inc_coins(player_id, -(amount as i64))
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

        let target_id = target.id.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.inc_coins(target_id, amount as i64).await.map_err(|e| {
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

        self.coins -= amount;
        target.coins += amount;
        cache.insert(self.username.clone(), self.clone());
        cache.insert(target.username.clone(), target.clone());

        Ok(())
    }

    pub async fn add_friend_request(
        &self,
        target: &mut Self,
        now: u64,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Player>>,
    ) -> Result<()> {
        let player_id = self.id.clone();
        let target_id = target.id.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.add_incoming_friend_request(player_id, target_id, now)
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

        target.incoming_friend_requests.insert(player_id, now);
        cache.insert(target.username.clone(), target.clone());

        Ok(())
    }

    pub async fn accept_friend_request(
        &mut self,
        sender: (u64, String),
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let player_id = self.id.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.remove_incoming_friend_request(player_id, sender.0)
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

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.add_friend(player_id, sender.0).await.map_err(|e| {
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

        self.incoming_friend_requests.remove(&sender.0);
        self.friends.insert(sender.0.clone());

        cache.insert(self.username.clone(), self.clone());

        if cache.contains_key(&sender.1) {
            let mut sender_player = cache.get(&sender.1).unwrap().clone();
            sender_player.friends.insert(self.id.clone());
            cache.insert(sender_player.username.clone(), sender_player);
        }

        Ok(())
    }

    pub async fn reject_friend_request(
        &mut self,
        sender: u64,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let player_id = self.id.clone();
        let sender_id = sender.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.remove_incoming_friend_request(player_id, sender_id)
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

        self.incoming_friend_requests.remove(&sender_id);
        cache.insert(self.username.clone(), self.clone());

        Ok(())
    }

    pub async fn remove_friend(
        &mut self,
        target: u64,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
        index: &Arc<DashMap<u64, String>>,
    ) -> Result<()> {
        let target = target.clone();
        let player_id = target.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.remove_friend(player_id, target).await.map_err(|e| {
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

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.remove_friend(target, player_id).await.map_err(|e| {
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

        self.friends.remove(&target);
        cache.insert(self.username.clone(), self.clone());

        let target_username = index.get(&target).unwrap().clone();

        if cache.contains_key(&target_username) {
            let mut target_player = cache.get(&target_username).unwrap().clone();
            target_player.friends.remove(&self.id);
            cache.insert(target_player.username.clone(), target_player);
        }

        Ok(())
    }

    pub async fn accept_team_request(
        &mut self,
        team_id: u64,
        now: u64,
        p_col: &Arc<dyn PlayerCollection>,
        t_col: &Arc<dyn TeamCollection>,
        p_cache: &Arc<DashMap<String, Self>>,
        t_cache: &Arc<DashMap<u64, Team>>,
    ) -> Result<()> {
        let t_col = t_col.clone();
        let player_id = self.id.clone();
        let team_id = team_id.clone();

        debug!(
            "Spawning a task to remove the incoming team request from player {}",
            self.username
        );
        task::spawn({
            let p_col = p_col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    p_col
                        .remove_incoming_team_request(player_id, team_id)
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

        task::spawn({
            let p_col = p_col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    p_col.set_team(player_id, team_id).await.map_err(|e| {
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

        debug!(
            "Spawning a task to add player {} into team's members",
            self.username
        );

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                t_col
                    .add_member(team_id, player_id, now)
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

        debug!("Updating the cache");

        self.incoming_team_requests.remove(&team_id);
        p_cache.insert(self.username.clone(), self.clone());

        debug!("Updating the team");

        let mut team: Team = t_cache.get(&team_id).unwrap().clone();
        team.members.insert(self.id.clone(), now.clone());
        t_cache.insert(team_id, team);

        debug!("Successfully updated team");

        Ok(())
    }

    pub async fn set_team(
        &mut self,
        team_id: u64,
        p_col: &Arc<dyn PlayerCollection>,
        p_cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let p_col = p_col.clone();
        let player_id = self.id.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                p_col.set_team(player_id, team_id).await.map_err(|e| {
                    error!("Retrying player update due to: {}", e);
                    e
                })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Player update permanently failed: {}", e);
            }
        });

        self.team = Some(team_id);
        p_cache.insert(self.username.clone(), self.clone());

        Ok(())
    }

    pub async fn reject_team_request(
        &mut self,
        team_id: u64,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> anyhow::Result<()> {
        let player_id = self.id;
        let team_id = team_id.clone();
        let col = col.clone();

        task::spawn({
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.remove_incoming_friend_request(player_id, team_id)
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
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let player_id = self.id.clone();
        let team_id = team_id.clone();
        let col = col.clone();

        task::spawn({
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.add_incoming_team_request(player_id, team_id, now)
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

    pub async fn add_kill(
        &mut self,
        kills: u64,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let col = col.clone();
        let player_id = self.id.clone();

        task::spawn({
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.add_kill(player_id, kills).await.map_err(|e| {
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

        self.kills += kills;
        cache.insert(self.username.clone(), self.clone());

        Ok(())
    }

    pub async fn add_death(
        &mut self,
        deaths: u64,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let col = col.clone();
        let player_id = self.id.clone();

        task::spawn({
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.add_death(player_id, deaths).await.map_err(|e| {
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

        self.deaths += deaths;
        cache.insert(self.username.clone(), self.clone());

        Ok(())
    }

    pub async fn add_blocks_placed(
        &mut self,
        blocks_placed: u64,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let col = col.clone();
        let player_id = self.id.clone();

        task::spawn({
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.add_blocks_placed(player_id, blocks_placed)
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

        self.blocks_placed += blocks_placed;
        cache.insert(self.username.clone(), self.clone());

        Ok(())
    }

    pub async fn add_blocks_broken(
        &mut self,
        blocks_broken: u64,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Self>>,
    ) -> Result<()> {
        let col = col.clone();
        let player_id = self.id.clone();

        task::spawn({
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.add_block_broken(player_id, blocks_broken)
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

        self.blocks_broken += blocks_broken;
        cache.insert(self.username.clone(), self.clone());

        Ok(())
    }

    pub fn get_rank_value(rank: &Rank) -> u64 {
        match rank {
            Rank::Newbie => 1,
            Rank::Member => 2,
            Rank::Vip => 3,
            Rank::Elite => 4,
            Rank::Legend => 5,
        }
    }
}
