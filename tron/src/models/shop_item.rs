use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShopItem {
    #[serde(rename = "_id")]
    pub id: String,
    pub item_id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub price: u32,
}
