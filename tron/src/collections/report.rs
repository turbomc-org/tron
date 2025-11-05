use crate::models::report::Report;
use async_trait::async_trait;
use futures::TryStreamExt;
use mockall::automock;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error;
use std::fmt::Debug;

#[automock]
#[async_trait]
pub trait ReportCollection: Send + Sync + Debug {
    async fn all(&self) -> Result<Vec<Report>, Error>;
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
