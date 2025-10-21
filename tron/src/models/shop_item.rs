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
    pub buy_price: u32,
    pub sell_price: u32,
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
