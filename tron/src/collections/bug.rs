use crate::models::bug::Bug;
use async_trait::async_trait;
use futures::TryStreamExt;
use mockall::automock;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error;
use std::fmt::Debug;

#[automock]
#[async_trait]
pub trait BugCollection: Send + Sync + Debug {
    async fn all(&self) -> Result<Vec<Bug>, Error>;
    async fn find_by_id(&self, id: u64) -> Result<Option<Bug>, Error>;
    async fn insert_one(&self, bug: &Bug) -> Result<(), Error>;
    async fn delete_one(&self, id: u64) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct MongoBugCollection {
    pub collection: Collection<Bug>,
}

#[async_trait]
impl BugCollection for MongoBugCollection {
    async fn all(&self) -> Result<Vec<Bug>, Error> {
        self.collection.find(doc! {}).await?.try_collect().await
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<Bug>, Error> {
        self.collection.find_one(doc! {"_id": id as i64}).await
    }

    async fn insert_one(&self, bug: &Bug) -> Result<(), Error> {
        self.collection.insert_one(bug).await?;
        Ok(())
    }

    async fn delete_one(&self, id: u64) -> Result<(), Error> {
        self.collection.delete_one(doc! {"_id": id as i64}).await?;
        Ok(())
    }
}
