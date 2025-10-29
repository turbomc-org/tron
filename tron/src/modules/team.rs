use crate::RETRY_STRATEGY;
use crate::collections::player::PlayerCollection;
use crate::collections::team::TeamCollection;
use crate::models::player::Player;
use crate::models::team::Team;
use crate::state::State;
use anyhow::Result;
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tracing::error;

impl Team {
    pub async fn insert(&self, col: &Arc<dyn TeamCollection>, state: &Arc<State>) -> Result<()> {
        let team_col = col.clone();
        let team = self.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                let team = team.clone();
                team_col.insert_one(&team).await.map_err(|e| {
                    error!("Retrying team update due to: {}", e);
                    e
                })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Team update permanently failed: {}", e);
            }
        });

        state.insert_team(self.clone());

        Ok(())
    }

    pub async fn add_member(
        &mut self,
        player: &mut Player,
        now: u64,
        p_col: &Arc<dyn PlayerCollection>,
        t_col: &Arc<dyn TeamCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let p_col = p_col.clone();
        let t_col = t_col.clone();
        let player_id = player.id;
        let team_id = self.id;

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

        self.members.insert(player.id.clone(), now);
        player.team = Some(self.id);

        state.insert_team(self.clone());
        state.insert_player(player.clone());

        Ok(())
    }

    pub async fn remove_member(
        &mut self,
        player: &mut Player,
        p_col: &Arc<dyn PlayerCollection>,
        t_col: &Arc<dyn TeamCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let p_col = p_col.clone();
        let t_col = t_col.clone();
        let player_id = player.id;
        let team_id = self.id;

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                p_col.unset_team(player_id, team_id).await.map_err(|e| {
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
                t_col.remove_member(team_id, player_id).await.map_err(|e| {
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

        state.insert_team(self.clone());
        state.insert_player(player.clone());

        Ok(())
    }

    pub async fn promote_player(
        &mut self,
        id: u64,
        col: &Arc<dyn TeamCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let team_id = self.id.clone();
        let player_id = id.clone();
        let team_col = col.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                team_col.set_leader(team_id, player_id).await.map_err(|e| {
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

        state.insert_team(self.clone());

        Ok(())
    }

    pub async fn set_open(
        &mut self,
        open: bool,
        col: &Arc<dyn TeamCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let team_col = col.clone();
        let team_id = self.id.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                team_col.set_open(team_id, open).await.map_err(|e| {
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

        state.insert_team(self.clone());

        Ok(())
    }
}
