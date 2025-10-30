use crate::models::team::Team;
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
pub trait TeamCollection: Send + Sync + Debug {
    async fn all(&self) -> Result<Vec<Team>, Error>;
    async fn indexes(&self) -> Result<HashMap<String, u64>, Error>;
    async fn find_by_id(&self, id: u64) -> Result<Option<Team>, Error>;
    async fn insert_one(&self, team: &Team) -> Result<(), Error>;
    async fn delete_one(&self, id: u64) -> Result<(), Error>;
    async fn add_member(&self, team_id: u64, player_id: u64, now: u64) -> Result<(), Error>;
    async fn remove_member(&self, team_id: u64, player_id: u64) -> Result<(), Error>;
    async fn set_leader(&self, team_id: u64, player_id: u64) -> Result<(), Error>;
    async fn set_open(&self, team_id: u64, open: bool) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct MongoTeamCollection {
    pub collection: Collection<Team>,
}

#[async_trait]
impl TeamCollection for MongoTeamCollection {
    async fn all(&self) -> Result<Vec<Team>, Error> {
        self.collection.find(doc! {}).await?.try_collect().await
    }

    async fn indexes(&self) -> Result<HashMap<String, u64>, Error> {
        let mut indexes = HashMap::new();

        #[derive(Serialize, Deserialize)]
        struct PartialResponse {
            #[serde(rename = "_id")]
            id: u64,
            name: String,
        }

        let partial_teams: Collection<PartialResponse> = self.collection.clone_with_type();
        let projection = doc! { "_id": 1, "name": 1  };
        let find_options = FindOptions::builder().projection(projection).build();

        let mut team_cursor = partial_teams
            .find(doc! {})
            .with_options(find_options)
            .await?;

        while let Some(team) = team_cursor.try_next().await? {
            indexes.insert(team.name, team.id);
        }

        Ok(indexes)
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<Team>, Error> {
        self.collection.find_one(doc! {"_id": id as i64}).await
    }

    async fn insert_one(&self, player: &Team) -> Result<(), Error> {
        self.collection.insert_one(player).await?;
        Ok(())
    }

    async fn delete_one(&self, id: u64) -> Result<(), Error> {
        self.collection.delete_one(doc! {"_id": id as i64}).await?;
        Ok(())
    }

    async fn add_member(&self, team_id: u64, player_id: u64, now: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": team_id as i64},
                doc! {
                    "$set": {
                        format!("members.{}", player_id): now as i64
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn remove_member(&self, team_id: u64, member_id: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": team_id as i64},
                doc! {
                    "$unset": {
                        format!("members.{}", member_id): ""
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn set_leader(&self, team_id: u64, player_id: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": team_id as i64},
                doc! {"$set": {"leader": player_id as i64}},
            )
            .await?;
        Ok(())
    }

    async fn set_open(&self, id: u64, open: bool) -> Result<(), Error> {
        self.collection
            .update_one(doc! {"_id": id as i64}, doc! {"$set": {"open": open}})
            .await?;
        Ok(())
    }
}
