use crate::models::prefix::Prefix;
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

#[async_trait]
#[automock]
pub trait PrefixCollection: Send + Sync + Debug {
    async fn all(&self) -> Result<Vec<Prefix>, Error>;
    async fn indexes(&self) -> Result<HashMap<String, u64>, Error>;
    async fn find_by_id(&self, id: u64) -> Result<Option<Prefix>, Error>;
    async fn insert_one(&self, prefix: &Prefix) -> Result<(), Error>;
    async fn delete_one(&self, prefix: u64) -> Result<(), Error>;
}

#[derive(Debug)]
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

    async fn indexes(&self) -> Result<HashMap<String, u64>, Error> {
        #[derive(Serialize, Deserialize)]
        struct PartialResponse {
            #[serde(rename = "_id")]
            id: u64,
            text: String,
        }

        let partial_prefixes: Collection<PartialResponse> = self.collection.clone_with_type();
        let projection = doc! { "_id": 1, "text": 1  };
        let find_options = FindOptions::builder().projection(projection).build();

        let mut prefix_cursor = partial_prefixes
            .find(doc! {})
            .with_options(find_options)
            .await?;

        let mut indexes = HashMap::new();
        while let Some(prefix) = prefix_cursor.try_next().await? {
            indexes.insert(prefix.text, prefix.id);
        }

        Ok(indexes)
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
