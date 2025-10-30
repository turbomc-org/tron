use crate::models::shop_item::ShopItem;
use async_trait::async_trait;
use futures::TryStreamExt;
use mockall::automock;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

#[automock]
#[async_trait]
pub trait ShopItemCollection: Send + Sync + Debug {
    async fn all(&self) -> Result<Vec<ShopItem>, Error>;
    async fn indexes(&self) -> Result<HashMap<(String, Vec<String>), u64>, Error>;
    async fn find_by_id(&self, id: u64) -> Result<Option<ShopItem>, Error>;
    async fn insert_one(&self, shop_item: &ShopItem) -> Result<(), Error>;
    async fn delete_one(&self, id: u64) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct MongoShopItemCollection {
    pub collection: Collection<ShopItem>,
}

#[async_trait]
impl ShopItemCollection for MongoShopItemCollection {
    async fn all(&self) -> Result<Vec<ShopItem>, Error> {
        self.collection.find(doc! {}).await?.try_collect().await
    }

    async fn indexes(&self) -> Result<HashMap<(String, Vec<String>), u64>, Error> {
        #[derive(Serialize, Deserialize)]
        struct PartialResponse {
            #[serde(rename = "_id")]
            id: u64,
            type_id: String,
            enchantments: Vec<String>,
        }

        let partial_shop_items: Collection<PartialResponse> = self.collection.clone_with_type();
        let projection = doc! { "_id": 1, "type_id": 1, "enchantments": 1 };
        let find_options = FindOptions::builder().projection(projection).build();

        let mut shop_item_cursor = partial_shop_items
            .find(doc! {})
            .with_options(find_options)
            .await?;

        let mut indexes = HashMap::new();
        while let Some(shop_item) = shop_item_cursor.try_next().await? {
            indexes.insert((shop_item.type_id, shop_item.enchantments), shop_item.id);
        }

        Ok(indexes)
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<ShopItem>, Error> {
        self.collection.find_one(doc! {"_id": id as i64}).await
    }

    async fn insert_one(&self, shop_item: &ShopItem) -> Result<(), Error> {
        self.collection.insert_one(shop_item.clone()).await?;
        Ok(())
    }

    async fn delete_one(&self, id: u64) -> Result<(), Error> {
        self.collection.delete_one(doc! {"_id": id as i64}).await?;
        Ok(())
    }
}
