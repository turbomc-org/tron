use crate::RETRY_STRATEGY;
use crate::collections::player::PlayerCollection;
use crate::collections::shop_item::ShopItemCollection;
use crate::models::player::Player;
use crate::models::shop_item::ShopItem;
use crate::state::State;
use anyhow::Result;
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tracing::error;

impl ShopItem {
    pub async fn insert(
        &self,
        col: &Arc<dyn ShopItemCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let col = col.clone();
        let shop_item = self.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                let shop_item = shop_item.clone();
                col.insert_one(&shop_item).await.map_err(|e| {
                    error!("Retrying team update due to: {}", e);
                    e
                })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Team update permanently failed: {}", e);
            }
        });

        state.insert_shop_item(self.clone()).await?;

        Ok(())
    }

    pub async fn remove(
        &self,
        col: &Arc<dyn ShopItemCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let col = col.clone();
        let item_id = self.id;

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                let item_id = item_id.clone();
                col.delete_one(item_id).await.map_err(|e| {
                    error!("Retrying team update due to: {}", e);
                    e
                })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Team update permanently failed: {}", e);
            }
        });

        state.remove_shop_item(&self).await?;

        Ok(())
    }

    pub async fn buy(
        &self,
        player: &mut Player,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
    ) -> Result<()> {
        let col = col.clone();
        let player_id = player.id.clone();
        let price = self.buy_price.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                col.inc_coins(player_id, -(price as i64))
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

        player.coins -= price;
        state.insert_player(player.clone()).await?;

        Ok(())
    }

    pub async fn sell(
        &self,
        quantity: u64,
        player: &mut Player,
        col: &Arc<dyn PlayerCollection>,
        state: &Arc<State>,
    ) -> Result<u64> {
        let col = col.clone();
        let player_id = player.id.clone();
        let price = self.sell_price.clone() * quantity;

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                col.inc_coins(player_id, price as i64).await.map_err(|e| {
                    error!("Retrying team update due to: {}", e);
                    e
                })
            })
            .await;

            if let Err(e) = retry_result {
                error!("Team update permanently failed: {}", e);
            }
        });

        player.coins += price;
        state.insert_player(player.clone()).await?;

        Ok(price)
    }
}
