use std::sync::Arc;

use crate::models::player::Player;
use anyhow::Result;
use tokio::task;
use tokio_retry::Retry;
use tracing::error;

use crate::{
    RETRY_STRATEGY,
    collections::{player::PlayerCollection, redeem::RedeemCollection},
    models::redeem::{Redeem, Reward},
    state::State,
};

impl Redeem {
    pub async fn insert(&self, col: &Arc<dyn RedeemCollection>, state: &Arc<State>) -> Result<()> {
        let redeem = self.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.insert_one(&redeem).await.map_err(|e| {
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

        state.insert_redeem(self.clone());

        Ok(())
    }

    pub async fn delete(&self, col: &Arc<dyn RedeemCollection>, state: &Arc<State>) -> Result<()> {
        let redeem_id = self.id.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.delete_one(redeem_id).await.map_err(|e| {
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

        state.remove_redeem(&redeem_id, &self.code);

        Ok(())
    }

    pub async fn redeem(
        &self,
        player: &mut Player,
        p_col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let player_id = player.id.clone();

        match self.reward {
            Reward::Coins(coins) => {
                task::spawn({
                    let p_col = p_col.clone();
                    async move {
                        let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                            p_col
                                .inc_coins(player_id.clone(), coins as i64)
                                .await
                                .map_err(|e| {
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

                player.coins += coins;
                state.inc_coins(player_id.clone(), coins as i64).await?;
            }
            Reward::Rank(rank) => {
                let p_col = p_col.clone();
                let player_id = player_id.clone();
                let rank = rank.clone();

                task::spawn(async move {
                    let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                        p_col
                            .set_rank(player_id.clone(), rank.clone())
                            .await
                            .map_err(|e| {
                                error!("Retrying rank update due to: {}", e);
                                e
                            })
                    })
                    .await;

                    if let Err(e) = retry_result {
                        error!("Rank update permanently failed: {}", e);
                    }
                });

                player.rank = rank.clone();
                state.insert_player(player.clone()).await?;
            }
        }

        player.redeemed_codes.insert(self.id.clone());
        state.insert_player(player.clone()).await?;

        Ok(())
    }
}
