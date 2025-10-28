use crate::models::shop_item::ShopItem;
use async_trait::async_trait;
use futures::TryStreamExt;
use mockall::automock;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error;

#[automock]
#[async_trait]
pub trait ShopItemCollection: Send + Sync {
    async fn all(&self) -> Result<Vec<ShopItem>, Error>;
    async fn find_by_id(&self, id: u64) -> Result<Option<ShopItem>, Error>;
    async fn insert_one(&self, shop_item: &ShopItem) -> Result<(), Error>;
    async fn delete_one(&self, id: u64) -> Result<(), Error>;
}

pub struct MongoShopItemCollection {
    pub collection: Collection<ShopItem>,
}

#[async_trait]
impl ShopItemCollection for MongoShopItemCollection {
    async fn all(&self) -> Result<Vec<ShopItem>, Error> {
        self.collection.find(doc! {}).await?.try_collect().await
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
