use crate::models::prefix::Prefix;
use async_trait::async_trait;
use futures::TryStreamExt;
use mockall::automock;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error;

#[async_trait]
#[automock]
pub trait PrefixCollection: Send + Sync {
    async fn all(&self) -> Result<Vec<Prefix>, Error>;
    async fn find_by_id(&self, id: u64) -> Result<Option<Prefix>, Error>;
    async fn insert_one(&self, prefix: &Prefix) -> Result<(), Error>;
    async fn delete_one(&self, prefix: u64) -> Result<(), Error>;
}

pub struct MongoPrefixCollection {
    pub collection: Collection<Prefix>,
}

impl MongoPrefixCollection {
    pub fn new(collection: Collection<Prefix>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl PrefixCollection for MongoPrefixCollection {
    async fn all(&self) -> Result<Vec<Prefix>, Error> {
        self.collection.find(doc! {}).await?.try_collect().await
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<Prefix>, Error> {
        self.collection.find_one(doc! {"_id": id as i64}).await
    }

    async fn insert_one(&self, prefix: &Prefix) -> Result<(), Error> {
        self.collection.insert_one(prefix.clone()).await?;
        Ok(())
    }

    async fn delete_one(&self, id: u64) -> Result<(), Error> {
        self.collection.delete_one(doc! {"_id": id as i64}).await?;
        Ok(())
    }
}
