use crate::models::report::Report;
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
pub trait ReportCollection: Send + Sync + Debug {
    async fn all(&self) -> Result<Vec<Report>, Error>;
    async fn indexes(&self) -> Result<HashSet<u64>, Error>;
    async fn find_by_id(&self, id: u64) -> Result<Option<Report>, Error>;
    async fn insert_one(&self, report: &Report) -> Result<(), Error>;
    async fn delete_one(&self, id: u64) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct MongoReportCollection {
    pub collection: Collection<Report>,
}

#[async_trait]
impl ReportCollection for MongoReportCollection {
    async fn all(&self) -> Result<Vec<Report>, Error> {
        self.collection.find(doc! {}).await?.try_collect().await
    }

    async fn indexes(&self) -> Result<HashSet<u64>, Error> {
        #[derive(Serialize, Deserialize)]
        struct PartialResponse {
            #[serde(rename = "_id")]
            id: u64,
        }

        let partial_prefixes: Collection<PartialResponse> = self.collection.clone_with_type();
        let projection = doc! { "_id": 1  };
        let find_options = FindOptions::builder().projection(projection).build();

        let mut report_cursor = partial_prefixes
            .find(doc! {})
            .with_options(find_options)
            .await?;

        let mut indexes = HashSet::new();
        while let Some(prefix) = report_cursor.try_next().await? {
            indexes.insert(prefix.id);
        }

        Ok(indexes)
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<Report>, Error> {
        self.collection.find_one(doc! {"_id": id as i64}).await
    }

    async fn insert_one(&self, report: &Report) -> Result<(), Error> {
        self.collection.insert_one(report).await?;
        Ok(())
    }

    async fn delete_one(&self, id: u64) -> Result<(), Error> {
        self.collection.delete_one(doc! {"_id": id as i64}).await?;
        Ok(())
    }
}
