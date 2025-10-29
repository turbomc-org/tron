use crate::RETRY_STRATEGY;
use crate::collections::player::PlayerCollection;
use crate::collections::prefix::PrefixCollection;
use crate::models::player::Player;
use crate::models::prefix::Prefix;
use crate::state::State;
use anyhow::Result;
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tonic::Status;
use tracing::error;

impl Prefix {
    pub async fn insert(&self, col: &Arc<dyn PrefixCollection>, state: &Arc<State>) -> Result<()> {
        let prefix = self.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.insert_one(&prefix).await.map_err(|e| {
                        error!("Retrying prefix insertion due to: {}", e);
                        e
                    })
                })
                .await;

                if let Err(e) = retry_result {
                    error!("Prefix insertion permanently failed: {}", e);
                }
            }
        });

        state.insert_prefix(self.clone());

        Ok(())
    }

    pub async fn delete(
        &self,
        col: &Arc<dyn PrefixCollection>,
        state: &Arc<State>,
    ) -> Result<(), Status> {
        let prefix_id = self.id.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.delete_one(prefix_id).await.map_err(|e| {
                        error!("Retrying prefix insertion due to: {}", e);
                        e
                    })
                })
                .await;

                if let Err(e) = retry_result {
                    error!("Prefix insertion permanently failed: {}", e);
                }
            }
        });

        state.remove_prefix(&self.id, &self.text);

        Ok(())
    }

    pub async fn buy(
        &self,
        player: &mut Player,
        player_col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
    ) -> Result<(), Status> {
        let player_id = player.id.clone();
        let price = self.price.clone();
        let prefix_id = self.id.clone();

        task::spawn({
            let player_col = player_col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    player_col
                        .inc_coins(player_id, -(price as i64))
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
            let player_col = player_col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    player_col
                        .add_prefix(player_id, prefix_id)
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

        player.coins -= price;
        player.prefixes.insert(self.id.clone());
        state.insert_player(player.clone());

        Ok(())
    }

    pub async fn select(
        &self,
        player: &mut Player,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let player_id = player.id.clone();
        let prefix_id = self.id.clone();
        let col = col.clone();

        task::spawn({
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.select_prefix(player_id.clone(), prefix_id.clone())
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

        player.selected_prefix = Some(prefix_id.clone());
        state.insert_player(player.clone());

        Ok(())
    }
}
