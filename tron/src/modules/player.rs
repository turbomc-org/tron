use crate::RETRY_STRATEGY;
use crate::collections::player::PlayerCollection;
use crate::collections::team::TeamCollection;
use crate::models::player::{Player, Rank};
use crate::models::team::Team;
use crate::state::State;
use anyhow::Result;
use anyhow::anyhow;
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tracing::error;

impl Player {
    pub async fn insert(&self, col: &Arc<dyn PlayerCollection>, state: &Arc<State>) -> Result<()> {
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

        state.insert_player(self.clone()).await?;

        Ok(())
    }

    pub async fn transfer_coins(
        &mut self,
        target: &mut Self,
        amount: u64,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let player_id = self.id.clone();
        let target_id = target.id.clone();

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
        state.inc_coins(player_id, -(amount as i64)).await?;
        state.inc_coins(target_id, amount as i64).await?;

        Ok(())
    }

    pub async fn add_friend_request(
        &self,
        target: &mut Self,
        now: u64,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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
        state.insert_player(target.clone()).await?;

        Ok(())
    }

    pub async fn accept_friend_request(
        &mut self,
        sender: (u64, String),
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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

        state.insert_player(self.clone()).await?;

        if state.active_players.contains_key(&sender.1) {
            let mut sender_player = state.get_active_player(&sender.1).await?.unwrap().clone();
            sender_player.friends.insert(self.id.clone());
            state.insert_player(sender_player).await?;
        }

        Ok(())
    }

    pub async fn reject_friend_request(
        &mut self,
        sender: u64,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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
        state.insert_player(self.clone()).await?;

        Ok(())
    }

    pub async fn remove_friend(
        &mut self,
        target: u64,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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
        state.insert_player(self.clone()).await?;

        let target_username = state.get_player_username(&target).await?.unwrap().clone();

        if state.active_players.contains_key(&target_username) {
            let mut target_player = state
                .get_active_player(&target_username)
                .await?
                .unwrap()
                .clone();
            target_player.friends.remove(&self.id);
            state.insert_player(target_player).await?;
        }

        Ok(())
    }

    pub async fn accept_team_request(
        &mut self,
        team_id: u64,
        now: u64,
        p_col: &Arc<dyn PlayerCollection>,
        t_col: &Arc<dyn TeamCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let t_col = t_col.clone();
        let player_id = self.id.clone();
        let team_id = team_id.clone();

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

        self.incoming_team_requests.remove(&team_id);
        state.insert_player(self.clone()).await?;

        let mut team: Team = state.get_team(team_id).await?.ok_or_else(|| {
            error!("Team not found");
            anyhow!("Team not found")
        })?;

        team.members.insert(self.id.clone(), now.clone());
        state.insert_team(team).await?;

        Ok(())
    }

    pub async fn set_team(
        &mut self,
        team_id: u64,
        p_col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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
        state.insert_player(self.clone()).await?;

        Ok(())
    }

    pub async fn reject_team_request(
        &mut self,
        team_id: u64,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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
        state.insert_player(self.clone()).await?;

        Ok(())
    }

    pub async fn add_team_invite(
        &mut self,
        team_id: u64,
        now: u64,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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
        state.insert_player(self.clone()).await?;

        Ok(())
    }

    pub async fn add_kill(
        &mut self,
        kills: u64,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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
        state.inc_kills(player_id, kills).await?;

        Ok(())
    }

    pub async fn add_death(
        &mut self,
        deaths: u64,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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
        state.inc_deaths(player_id, deaths).await?;

        Ok(())
    }

    pub async fn add_blocks_placed(
        &mut self,
        blocks_placed: u64,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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
        state.insert_player(self.clone()).await?;

        Ok(())
    }

    pub async fn add_blocks_broken(
        &mut self,
        blocks_broken: u64,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
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
        state.insert_player(self.clone()).await?;

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
