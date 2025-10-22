use crate::models::leaderboards::Leaderboards;
use crate::models::player::Player;
use anyhow::Result;
use futures::TryStreamExt;
use mongodb::Collection;
use mongodb::bson::doc;
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
}
