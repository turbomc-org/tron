use crate::models::player::Player;
use crate::models::prefix::Prefix;
use crate::models::shop_item::ShopItem;
use crate::models::team::Team;
use crate::models::{databases::Databases, leaderboards::Leaderboards};
use dashmap::DashMap;
use futures::TryStreamExt;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

pub mod player;
pub mod prefix;
pub mod shop_item;
pub mod team;

pub struct Cache {
    pub active_players: Arc<DashMap<String, Player>>,
    pub shop_items: Arc<DashMap<u64, ShopItem>>,
    pub teams: Arc<DashMap<u64, Team>>,
    pub prefixes: Arc<DashMap<u64, Prefix>>,
    pub player_indexes: Arc<DashMap<u64, String>>,
    pub leaderboards: Leaderboards,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            active_players: Arc::new(DashMap::new()),
            shop_items: Arc::new(DashMap::new()),
            teams: Arc::new(DashMap::new()),
            prefixes: Arc::new(DashMap::new()),
            player_indexes: Arc::new(DashMap::new()),
            leaderboards: Leaderboards::new(),
        }
    }

    pub async fn init(db: &Databases) -> anyhow::Result<Self> {
        let cache = Self::new();

        info!("Populating shop items to cache from mongodb");
        let mut shop_cursor = db.shop_items.find(doc! {}).await?;
        while let Some(doc) = shop_cursor.try_next().await? {
            cache.shop_items.insert(doc.id, doc);
        }

        info!("Populating teams to cache from mongodb");
        let mut team_cursor = db.teams.find(doc! {}).await?;
        while let Some(doc) = team_cursor.try_next().await? {
            cache.teams.insert(doc.id, doc);
        }

        info!("Populating prefixes to cache from mongodb");
        let mut prefix_cursor = db.prefixes.find(doc! {}).await?;
        while let Some(doc) = prefix_cursor.try_next().await? {
            cache.prefixes.insert(doc.id, doc);
        }

        info!("Populating players indexes from mongodb");

        #[derive(Serialize, Deserialize)]
        struct PartialResponse {
            #[serde(rename = "_id")]
            id: u64,
            username: String,
        }

        let partial_players: Collection<PartialResponse> = db.players.clone_with_type();
        let projection = doc! { "_id": 1, "username": 1  };
        let find_options = FindOptions::builder().projection(projection).build();

        let mut user_cursor = partial_players
            .find(doc! {})
            .with_options(find_options)
            .await?;

        while let Some(doc) = user_cursor.try_next().await? {
            cache.player_indexes.insert(doc.id, doc.username);
        }

        Ok(cache)
    }
}
