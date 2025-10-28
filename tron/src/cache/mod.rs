use crate::bridge::{MessageResponse, ServerSendMessageResponse, ServerSendTitleResponse};
use crate::collections::Collections;
use crate::models::leaderboards::Leaderboards;
use crate::models::player::Player;
use crate::models::prefix::Prefix;
use crate::models::servers::Servers;
use crate::models::shop_item::ShopItem;
use crate::models::team::Team;
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::Status;
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
    pub team_indexes: Arc<DashMap<String, u64>>,
    pub leaderboards: Leaderboards,
    pub servers: Servers,
    pub message_clients: Arc<DashMap<u64, mpsc::Sender<Result<MessageResponse, Status>>>>,
    pub send_message_clients:
        Arc<DashMap<u64, mpsc::Sender<Result<ServerSendMessageResponse, Status>>>>,
    pub send_title_clients:
        Arc<DashMap<u64, mpsc::Sender<Result<ServerSendTitleResponse, Status>>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            active_players: Arc::new(DashMap::new()),
            shop_items: Arc::new(DashMap::new()),
            teams: Arc::new(DashMap::new()),
            prefixes: Arc::new(DashMap::new()),
            player_indexes: Arc::new(DashMap::new()),
            team_indexes: Arc::new(DashMap::new()),
            leaderboards: Leaderboards::new(),
            servers: Servers::new(),
            send_message_clients: Arc::new(DashMap::new()),
            send_title_clients: Arc::new(DashMap::new()),
            message_clients: Arc::new(DashMap::new()),
        }
    }

    pub async fn init(cols: &Collections) -> anyhow::Result<Self> {
        let cache = Self::new();

        info!("Populating shop items to cache from mongodb");
        let shop_items: Vec<ShopItem> = cols.shop_items.all().await?;
        for item in shop_items {
            cache.shop_items.insert(item.id, item);
        }

        info!("Populating teams to cache from mongodb");
        let teams: Vec<Team> = cols.teams.all().await?;
        for team in teams {
            cache.teams.insert(team.id, team);
        }

        info!("Populating prefixes to cache from mongodb");
        let prefixes: Vec<Prefix> = cols.prefixes.all().await?;
        for prefix in prefixes {
            cache.prefixes.insert(prefix.id, prefix);
        }

        info!("Populating players indexes from mongodb");
        let indexes: HashMap<String, u64> = cols.teams.indexes().await?;
        for (name, id) in indexes {
            cache.team_indexes.insert(name, id);
        }

        info!("Populating teams indexes from mongodb");

        Ok(cache)
    }
}
