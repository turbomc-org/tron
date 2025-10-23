use crate::GENERATOR;
use crate::bridge::{
    Item as CompiledItem, ItemType as CompiledItemType, Rarity as CompiledRarity,
    ShopItem as CompiledShopItem,
};
use crate::models::player::Player;
use anyhow::Result;
use bincode::{Decode, Encode};
use dashmap::DashMap;
use mongodb::Collection;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tracing::error;

use crate::RETRY_STRATEGY;

#[derive(Serialize, Deserialize, Encode, Decode, Clone)]
pub struct ShopItem {
    #[serde(rename = "_id")]
    pub id: u64,
    pub item: Item,
    pub name: String,
    pub description: String,
    pub category: String,
    pub buy_price: u64,
    pub sell_price: u64,
    pub rarity: Rarity,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone)]
pub struct Item {
    pub id: String,
    pub item_type: ItemType,
    pub enchantments: HashSet<String>,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone)]
pub enum ItemType {
    Tool,
    Block,
    Armor,
    Food,
    Misc,
    Container,
}

impl ShopItem {
    pub fn new(
        item_id: String,
        item_type: ItemType,
        name: String,
        description: String,
        category: String,
        enchantments: HashSet<String>,
        rarity: Rarity,
        buy_price: u64,
        sell_price: u64,
    ) -> Self {
        Self {
            id: GENERATOR.generate(),
            item: Item {
                id: item_id,
                item_type,
                enchantments,
            },
            name,
            description,
            category,
            buy_price,
            sell_price,
            rarity,
        }
    }

    pub async fn insert(
        &self,
        shop_col: &Collection<Self>,
        cache: &Arc<DashMap<u64, Self>>,
    ) -> Result<()> {
        let shop_col = shop_col.clone();
        let shop_item = self.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                let shop_item = shop_item.clone();
                shop_col.insert_one(shop_item).await.map_err(|e| {
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
        shop_col: &Collection<Self>,
        cache: &Arc<DashMap<u64, Self>>,
    ) -> Result<()> {
        let shop_col = shop_col.clone();
        let item_id = self.id;

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                let item_id = item_id.clone();
                shop_col
                    .delete_one(doc! {"_id": item_id as i64})
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

        cache.remove(&self.id);

        Ok(())
    }

    pub async fn buy(
        &self,
        player: &mut Player,
        col: &Collection<Player>,
        cache: &Arc<DashMap<String, Player>>,
    ) -> Result<()> {
        let col = col.clone();
        let player_id = player.id.clone();
        let price = self.buy_price.clone();

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                col.update_one(
                    doc! {"_id": player_id as i64},
                    doc! {"$inc": {"coins": -(price as i64)}},
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

        player.coins -= price;
        cache.insert(player.username.clone(), player.clone());

        Ok(())
    }

    pub async fn sell(
        &self,
        quantity: u64,
        player: &mut Player,
        col: &Collection<Player>,
        cache: &Arc<DashMap<String, Player>>,
    ) -> Result<u64> {
        let col = col.clone();
        let player_id = player.id.clone();
        let price = self.sell_price.clone() * quantity;

        task::spawn(async move {
            let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                col.update_one(
                    doc! {"_id": player_id as i64},
                    doc! {"$inc": {"coins": price as i64}},
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

        player.coins += price;
        cache.insert(player.username.clone(), player.clone());

        Ok(price)
    }

    pub fn convert_shop_item(original: ShopItem) -> CompiledShopItem {
        CompiledShopItem {
            id: original.id,
            item: Some(Self::convert_item(original.item)),
            name: original.name.into(),
            description: original.description.into(),
            category: original.category.into(),
            buy_price: original.buy_price,
            sell_price: original.sell_price,
            rarity: match original.rarity {
                Rarity::Common => CompiledRarity::Common as i32,
                Rarity::Uncommon => CompiledRarity::Uncommon as i32,
                Rarity::Rare => CompiledRarity::Rare as i32,
                Rarity::Epic => CompiledRarity::Epic as i32,
                Rarity::Legendary => CompiledRarity::Legendary as i32,
            },
        }
    }

    fn convert_item(original: Item) -> CompiledItem {
        CompiledItem {
            id: original.id.into(),
            item_type: match original.item_type {
                ItemType::Tool => CompiledItemType::Tool as i32,
                ItemType::Block => CompiledItemType::Block as i32,
                ItemType::Armor => CompiledItemType::Armor as i32,
                ItemType::Food => CompiledItemType::Food as i32,
                ItemType::Misc => CompiledItemType::Misc as i32,
                ItemType::Container => CompiledItemType::Container as i32,
            },
            enchantments: original.enchantments.into_iter().collect(),
        }
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
