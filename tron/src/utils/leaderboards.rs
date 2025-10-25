use crate::models::leaderboards::Leaderboards;
use crate::models::player::Player;
use crate::models::team::Team;
use anyhow::Result;
use futures::TryStreamExt;
use mongodb::Collection;
use mongodb::bson::{doc, from_document};
use mongodb::options::AggregateOptions;
use mongodb::options::FindOptions;

impl Leaderboards {
    pub async fn get_players(
        col: &Collection<Player>,
        sort_field: &str,
        limit: Option<i64>,
    ) -> Result<Vec<Player>> {
        let sort_order = match sort_field {
            "kills" => doc! { "kills": -1 },
            "deaths" => doc! { "deaths": -1 },
            "coins" => doc! { "coins": -1 },
            _ => doc! {},
        };

        let find_options = FindOptions::builder()
            .sort(sort_order)
            .limit(limit.unwrap_or(10))
            .build();

        let cursor = col.find(doc! {}).with_options(find_options).await?;

        let players: Vec<Player> = cursor.try_collect().await?;
        Ok(players)
    }

    pub async fn get_players_kd(
        col: &Collection<Player>,
        limit: Option<i64>,
    ) -> Result<Vec<Player>> {
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
            doc! { "$limit": limit.unwrap_or(10) },
        ];

        let aggregate_options = AggregateOptions::builder().build();

        let mut cursor = col
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

    pub async fn get_players_overall(
        col: &Collection<Player>,
        limit: Option<i64>,
    ) -> Result<Vec<Player>> {
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
            doc! { "$limit": limit.unwrap_or(10) },
        ];

        let aggregate_options = AggregateOptions::builder().build();

        let mut cursor = col
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

    pub async fn get_teams_overall(
        team_col: &Collection<Team>,
        player_col: &Collection<Player>,
        limit: Option<i64>,
    ) -> Result<Vec<(Team, f64)>> {
        let mut teams_cursor = team_col.find(doc! {}).await?;
        let mut leaderboard: Vec<(Team, f64)> = Vec::new();

        while let Some(team) = teams_cursor.try_next().await? {
            let member_ids: Vec<i64> = team.members.keys().cloned().map(|v| v as i64).collect();
            if member_ids.is_empty() {
                continue;
            }

            let filter = doc! { "_id": { "$in": &member_ids } };
            let mut cursor = player_col.find(filter).await?;

            let mut total_score = 0.0;
            let mut member_count = 0;

            while let Some(player) = cursor.try_next().await? {
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
}
