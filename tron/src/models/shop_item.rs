use crate::GENERATOR;
use crate::bridge::{Rarity as CompiledRarity, ShopItem as CompiledShopItem};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShopItem {
    #[serde(rename = "_id")]
    pub id: u64,
    pub type_id: String,
    pub enchantments: HashSet<String>,
    pub name: String,
    pub description: String,
    pub category: String,
    pub buy_price: u64,
    pub sell_price: u64,
    pub rarity: Rarity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl ShopItem {
    pub fn new(
        type_id: String,
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
            type_id,
            name,
            enchantments,
            description,
            category,
            buy_price,
            sell_price,
            rarity,
        }
    }

    pub fn convert_shop_item(original: ShopItem) -> CompiledShopItem {
        CompiledShopItem {
            id: original.id,
            type_id: original.type_id,
            enchantments: original
                .enchantments
                .into_iter()
                .map(|e| e.into())
                .collect(),
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
}
