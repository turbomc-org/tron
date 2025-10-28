use crate::GENERATOR;
use crate::bridge::{
    Item as CompiledItem, ItemType as CompiledItemType, Rarity as CompiledRarity,
    ShopItem as CompiledShopItem,
};
use bincode::{Decode, Encode};
use mongodb::bson::doc;
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
}
