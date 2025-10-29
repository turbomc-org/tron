use crate::collections::team::TeamCollection;
use crate::models::player::Player;
use crate::models::team::Team;
use async_trait::async_trait;
use futures::TryStreamExt;
use mockall::automock;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::bson::from_document;
use mongodb::error::Error;
use mongodb::options::AggregateOptions;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait PlayerCollection: Send + Sync {
    async fn all(&self) -> Result<Vec<Player>, Error>;
    async fn indexes(&self) -> Result<HashMap<u64, String>, Error>;
    async fn find_by_username(&self, username: &str) -> Result<Option<Player>, Error>;
    async fn find_by_discord_id(&self, discord_id: u64) -> Result<Option<Player>, Error>;
    async fn insert_one(&self, player: &Player) -> Result<(), Error>;
    async fn delete_one(&self, player: &Player) -> Result<(), Error>;
    async fn inc_coins(&self, id: u64, amount: i64) -> Result<(), Error>;
    async fn add_incoming_friend_request(
        &self,
        id: u64,
        target: u64,
        now: u64,
    ) -> Result<(), Error>;
    async fn remove_incoming_friend_request(&self, id: u64, target: u64) -> Result<(), Error>;
    async fn add_incoming_team_request(&self, id: u64, team: u64, now: u64) -> Result<(), Error>;
    async fn remove_incoming_team_request(&self, id: u64, team: u64) -> Result<(), Error>;
    async fn add_friend(&self, id: u64, target: u64) -> Result<(), Error>;
    async fn remove_friend(&self, id: u64, target: u64) -> Result<(), Error>;
    async fn add_death(&self, id: u64, n: u64) -> Result<(), Error>;
    async fn add_kill(&self, id: u64, n: u64) -> Result<(), Error>;
    async fn add_blocks_placed(&self, id: u64, n: u64) -> Result<(), Error>;
    async fn add_block_broken(&self, id: u64, n: u64) -> Result<(), Error>;
    async fn set_team(&self, id: u64, team: u64) -> Result<(), Error>;
    async fn unset_team(&self, id: u64, team: u64) -> Result<(), Error>;
    async fn get_leaderboard(
        &self,
        sort_field: &str,
        limit: Option<u64>,
    ) -> Result<Vec<Player>, Error>;
    async fn get_leaderboard_kda(&self, limit: Option<u64>) -> Result<Vec<Player>, Error>;
    async fn get_leaderboard_overall(&self, limit: Option<u64>) -> Result<Vec<Player>, Error>;
    async fn get_leaderboard_team(
        &self,
        team_col: &Arc<dyn TeamCollection>,
        limit: Option<u64>,
    ) -> Result<Vec<(Team, f64)>, Error>;
    async fn get_members(&self, members_id: &Vec<i64>) -> Result<Vec<Player>, Error>;
    async fn add_prefix(&self, player_id: u64, prefix_id: u64) -> Result<(), Error>;
    async fn select_prefix(&self, player_id: u64, prefix_id: u64) -> Result<(), Error>;
}

pub struct MongoPlayerCollection {
    pub collection: Collection<Player>,
}

#[async_trait]
impl PlayerCollection for MongoPlayerCollection {
    async fn all(&self) -> Result<Vec<Player>, Error> {
        self.collection.find(doc! {}).await?.try_collect().await
    }

    async fn indexes(&self) -> Result<HashMap<u64, String>, Error> {
        #[derive(Serialize, Deserialize)]
        struct PartialResponse {
            #[serde(rename = "_id")]
            id: u64,
            username: String,
        }

        let partial_players: Collection<PartialResponse> = self.collection.clone_with_type();
        let projection = doc! { "_id": 1, "username": 1  };
        let find_options = FindOptions::builder().projection(projection).build();

        let mut user_cursor = partial_players
            .find(doc! {})
            .with_options(find_options)
            .await?;

        let mut indexes = HashMap::new();
        while let Some(user) = user_cursor.try_next().await? {
            indexes.insert(user.id, user.username);
        }

        Ok(indexes)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<Player>, Error> {
        self.collection
            .find_one(doc! { "username": username })
            .await
    }

    async fn find_by_discord_id(&self, discord_id: u64) -> Result<Option<Player>, Error> {
        self.collection
            .find_one(doc! { "discord_id": discord_id as i64})
            .await
    }

    async fn insert_one(&self, player: &Player) -> Result<(), Error> {
        self.collection.insert_one(player).await?;
        Ok(())
    }

    async fn delete_one(&self, player: &Player) -> Result<(), Error> {
        self.collection
            .delete_one(doc! { "_id": player.id as i64 })
            .await?;
        Ok(())
    }

    async fn inc_coins(&self, id: u64, amount: i64) -> Result<(), Error> {
        self.collection
            .update_one(doc! {"_id": id as i64}, doc! {"$inc": {"coins": amount}})
            .await?;
        Ok(())
    }

    async fn add_incoming_friend_request(
        &self,
        id: u64,
        target: u64,
        now: u64,
    ) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": target as i64},
                doc! {
                    "$set": {
                        format!("incoming_friend_requests.{}", id): now as i64
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn remove_incoming_friend_request(&self, id: u64, target: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": id as i64},
                doc! {
                    "$unset": {
                        format!("incoming_friend_requests.{}", target): ""
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn add_incoming_team_request(
        &self,
        player_id: u64,
        team_id: u64,
        now: u64,
    ) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": player_id as i64},
                doc! {
                    "$set": {
                           format!("incoming_team_requests.{}", team_id): now as i64
                       }
                },
            )
            .await?;
        Ok(())
    }

    async fn remove_incoming_team_request(
        &self,
        player_id: u64,
        team_id: u64,
    ) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": player_id as i64},
                doc! {
                    "$unset": {
                        format!("incoming_team_requests.{}", team_id): ""
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn add_friend(&self, id: u64, target: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": id as i64},
                doc! {
                    "$addToSet": {
                        "friends": target as i64
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn remove_friend(&self, id: u64, target: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": id as i64},
                doc! {
                    "$pull": {
                        "friends": target as i64
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn add_death(&self, id: u64, n: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": id as i64},
                doc! {
                    "$inc": {
                        "deaths": n as i64
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn add_kill(&self, id: u64, n: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": id as i64},
                doc! {
                    "$inc": {
                        "kills": n as i64
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn add_blocks_placed(&self, id: u64, n: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": id as i64},
                doc! {
                    "$inc": {
                        "blocks_placed": n as i64
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn add_block_broken(&self, id: u64, n: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": id as i64},
                doc! {
                    "$inc": {
                        "blocks_broken": n as i64
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn set_team(&self, id: u64, team: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": id as i64},
                doc! {
                    "$set": {
                        "team": team as i64
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn unset_team(&self, id: u64, team: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": id as i64},
                doc! {
                    "$unset": {
                        "team": team as i64
                    }
                },
            )
            .await?;
        Ok(())
    }

    async fn get_leaderboard(
        &self,
        sort_field: &str,
        limit: Option<u64>,
    ) -> Result<Vec<Player>, Error> {
        let sort_order = match sort_field {
            "kills" => doc! { "kills": -1 },
            "deaths" => doc! { "deaths": -1 },
            "coins" => doc! { "coins": -1 },
            _ => doc! {},
        };

        let find_options = FindOptions::builder()
            .sort(sort_order)
            .limit(limit.unwrap_or(10) as i64)
            .build();

        let cursor = self
            .collection
            .find(doc! {})
            .with_options(find_options)
            .await?;

        let players: Vec<Player> = cursor.try_collect().await?;
        Ok(players)
    }

    async fn get_leaderboard_overall(&self, limit: Option<u64>) -> Result<Vec<Player>, Error> {
        let pipeline = vec![
            doc! {
                "$addFields": {
                    "kd_ratio": {
                        "$cond": {
                            "if": { "$eq": ["$deaths", 0] },
                            "then": "$kills",
                            "else": { "$divide": ["$kills", "$deaths"] }
                        }
                    }
                }
            },
            doc! {
                "$addFields": {
                    "rank_value": {
                        "$switch": {
                            "branches": [
                                { "case": { "$eq": ["$rank", "Newbie"] }, "then": 1 },
                                { "case": { "$eq": ["$rank", "Member"] }, "then": 2 },
                                { "case": { "$eq": ["$rank", "Vip"] }, "then": 3 },
                                { "case": { "$eq": ["$rank", "Elite"] }, "then": 4 },
                                { "case": { "$eq": ["$rank", "Legend"] }, "then": 5 }
                            ],
                            "default": 0
                        }
                    }
                }
            },
            doc! {
                "$addFields": {
                    "overall_score": {
                        "$add": [
                            "$kills",
                            { "$multiply": ["$kd_ratio", 100.0] },
                            "$coins",
                            { "$multiply": ["$rank_value", 1000.0] },
                            "$vault_count"
                        ]
                    }
                }
            },
            doc! { "$sort": { "overall_score": -1 } },
            doc! { "$limit": limit.unwrap_or(10)  as i64},
        ];

        let aggregate_options = AggregateOptions::builder().build();

        let mut cursor = self
            .collection
            .aggregate(pipeline)
            .with_options(aggregate_options)
            .await?;

        let mut players: Vec<Player> = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            let player: Player = from_document(doc)?;
            players.push(player);
        }

        Ok(players)
    }

    async fn get_leaderboard_kda(&self, limit: Option<u64>) -> Result<Vec<Player>, Error> {
        let pipeline = vec![
            doc! {
                "$addFields": {
                    "kd_ratio": {
                        "$cond": {
                            "if": { "$eq": ["$deaths", 0] },
                            "then": "$kills",
                            "else": { "$divide": ["$kills", "$deaths"] }
                        }
                    }
                }
            },
            doc! { "$sort": { "kd_ratio": -1 } },
            doc! { "$limit": limit.unwrap_or(10) as i64 },
        ];

        let aggregate_options = AggregateOptions::builder().build();

        let mut cursor = self
            .collection
            .aggregate(pipeline)
            .with_options(aggregate_options)
            .await?;

        let mut players: Vec<Player> = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            let player: Player = from_document(doc)?;
            players.push(player);
        }

        Ok(players)
    }

    async fn get_members(&self, members_id: &Vec<i64>) -> Result<Vec<Player>, Error> {
        let filter = doc! { "_id": { "$in": members_id } };
        self.collection.find(filter).await?.try_collect().await
    }

    async fn get_leaderboard_team(
        &self,
        team_col: &Arc<dyn TeamCollection>,
        limit: Option<u64>,
    ) -> Result<Vec<(Team, f64)>, Error> {
        let teams = team_col.all().await?;
        let mut leaderboard: Vec<(Team, f64)> = Vec::new();

        for team in teams {
            let member_ids: Vec<i64> = team.members.keys().cloned().map(|v| v as i64).collect();
            if member_ids.is_empty() {
                continue;
            }

            let members = self.get_members(&member_ids).await?;

            let mut total_score = 0.0;
            let mut member_count = 0;

            for player in members {
                let kd_ratio = if player.deaths == 0 {
                    player.kills as f64
                } else {
                    player.kills as f64 / player.deaths as f64
                };

                let rank_value = Player::get_rank_value(&player.rank);

                let score = player.kills as f64
                    + (kd_ratio * 100.0)
                    + player.coins as f64
                    + (rank_value as f64 * 1000.0)
                    + player.vault_count as f64;

                total_score += score;
                member_count += 1;
            }

            if member_count == 0 {
                continue;
            }

            let avg_score = total_score / member_count as f64;
            leaderboard.push((team, avg_score));
        }

        leaderboard.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let limit = limit.unwrap_or(10) as usize;
        leaderboard.truncate(limit);

        Ok(leaderboard)
    }

    async fn add_prefix(&self, player_id: u64, prefix_id: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {
                    "_id": player_id as i64,
                },
                doc! {
                    "$addToSet": {
                        "prefixes": prefix_id as i64
                    }
                },
            )
            .await?;

        Ok(())
    }

    async fn select_prefix(&self, player_id: u64, prefix_id: u64) -> Result<(), Error> {
        self.collection
            .update_one(
                doc! {"_id": player_id as i64},
                doc! {
                    "$set": {
                        "selected_prefix": prefix_id as i64
                    }
                },
            )
            .await?;

        Ok(())
    }
}
