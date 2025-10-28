use crate::RETRY_STRATEGY;
use crate::collections::player::PlayerCollection;
use crate::collections::shop_item::ShopItemCollection;
use crate::models::player::Player;
use crate::models::shop_item::ShopItem;
use anyhow::Result;
use dashmap::DashMap;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tracing::error;

impl ShopItem {
    pub async fn insert(
        &self,
        col: &Arc<dyn ShopItemCollection>,
        cache: &Arc<DashMap<u64, Self>>,
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

        cache.insert(self.id, self.clone());

        Ok(())
    }

    pub async fn remove(
        &self,
        col: &Arc<dyn ShopItemCollection>,
        cache: &Arc<DashMap<u64, Self>>,
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

        cache.remove(&self.id);

        Ok(())
    }

    pub async fn buy(
        &self,
        player: &mut Player,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Player>>,
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
        cache.insert(player.username.clone(), player.clone());

        Ok(())
    }

    pub async fn sell(
        &self,
        quantity: u64,
        player: &mut Player,
        col: &Arc<dyn PlayerCollection>,
        cache: &Arc<DashMap<String, Player>>,
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
        cache.insert(player.username.clone(), player.clone());

        Ok(price)
    }

    pub fn find_shop_item_by_id_and_enchantments(
        cache: &Arc<DashMap<u64, ShopItem>>,
        item_id: &str,
        enchantments: &HashSet<String>,
    ) -> Option<Self> {
        cache
            .iter()
            .find(|entry| {
                let shop_item = entry.value();
                shop_item.item.id == item_id && enchantments.is_subset(&shop_item.item.enchantments)
            })
            .map(|entry| entry.value().clone())
    }
}
