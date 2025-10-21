use crate::GENERATOR;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
}
