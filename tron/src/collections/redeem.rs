use crate::models::redeem::Redeem;
use async_trait::async_trait;
use futures::TryStreamExt;
use mockall::automock;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;

#[automock]
#[async_trait]
pub trait RedeemCollection: Send + Sync + Debug {
    async fn all(&self) -> Result<Vec<Redeem>, Error>;
    async fn indexes(&self) -> Result<HashSet<u64>, Error>;
    async fn find_by_id(&self, id: u64) -> Result<Option<Redeem>, Error>;
    async fn insert_one(&self, redeem: &Redeem) -> Result<(), Error>;
    async fn delete_one(&self, id: u64) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct MongoRedeemCollection {
    pub collection: Collection<Redeem>,
}

#[async_trait]
impl RedeemCollection for MongoRedeemCollection {
    async fn all(&self) -> Result<Vec<Redeem>, Error> {
        self.collection.find(doc! {}).await?.try_collect().await
    }

    async fn indexes(&self) -> Result<HashSet<u64>, Error> {
        #[derive(Serialize, Deserialize)]
        struct PartialResponse {
            #[serde(rename = "_id")]
            id: u64,
            code: String,
        }

        let partial_prefixes: Collection<PartialResponse> = self.collection.clone_with_type();
        let projection = doc! { "_id": 1, "code": 1 };
        let find_options = FindOptions::builder().projection(projection).build();

        let mut bug_cursor = partial_prefixes
            .find(doc! {})
            .with_options(find_options)
            .await?;

        let mut indexes = HashSet::new();
        while let Some(prefix) = bug_cursor.try_next().await? {
            indexes.insert(prefix.id);
        }

        Ok(indexes)
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<Redeem>, Error> {
        self.collection.find_one(doc! {"_id": id as i64}).await
    }

    async fn insert_one(&self, redeem: &Redeem) -> Result<(), Error> {
        self.collection.insert_one(redeem).await?;
        Ok(())
    }

    async fn delete_one(&self, id: u64) -> Result<(), Error> {
        self.collection.delete_one(doc! {"_id": id as i64}).await?;
        Ok(())
    }
}
