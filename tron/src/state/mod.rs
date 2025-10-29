use crate::bridge::{MessageResponse, ServerSendMessageResponse, ServerSendTitleResponse};
use crate::collections::Collections;
use crate::models::player::Player;
use crate::models::prefix::Prefix;
use crate::models::shop_item::ShopItem;
use crate::models::team::Team;
use dashmap::DashMap;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tonic::Status;
use tracing::info;

pub mod player;
pub mod prefix;
pub mod shop_item;
pub mod team;

pub struct State {
    pub active_players: DashMap<String, Player>,
    pub shop_items: DashMap<u64, ShopItem>,
    pub teams: DashMap<u64, Team>,
    pub prefixes: DashMap<u64, Prefix>,
    pub player_indexes: DashMap<u64, String>,
    pub team_indexes: DashMap<String, u64>,
    pub prefix_indexes: DashMap<String, u64>,
    pub shop_item_indexes: DashMap<(String, Vec<String>), u64>,
    pub message_clients: DashMap<u64, mpsc::Sender<Result<MessageResponse, Status>>>,
    pub send_message_clients: DashMap<u64, mpsc::Sender<Result<ServerSendMessageResponse, Status>>>,
    pub send_title_clients: DashMap<u64, mpsc::Sender<Result<ServerSendTitleResponse, Status>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            active_players: DashMap::new(),
            shop_items: DashMap::new(),
            teams: DashMap::new(),
            prefixes: DashMap::new(),
            player_indexes: DashMap::new(),
            team_indexes: DashMap::new(),
            prefix_indexes: DashMap::new(),
            shop_item_indexes: DashMap::new(),
            send_message_clients: DashMap::new(),
            send_title_clients: DashMap::new(),
            message_clients: DashMap::new(),
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
        let indexes: HashMap<u64, String> = cols
            .players
            .indexes()
            .await
            .expect("Failed to populate player indexes");
        for (id, username) in indexes {
            cache.player_indexes.insert(id, username);
        }

        info!("Populating teams indexes from mongodb");
        let indexes: HashMap<String, u64> = cols
            .teams
            .indexes()
            .await
            .expect("Failed to populate team indexes");
        for (name, id) in indexes {
            cache.team_indexes.insert(name, id);
        }

        info!("Populating prefix indexes from mongodb");
        let indexes: HashMap<String, u64> = cols
            .prefixes
            .indexes()
            .await
            .expect("Failed to populate prefix indexes");
        for (text, id) in indexes {
            cache.prefix_indexes.insert(text, id);
        }

        info!("Populating shop_item indexes from mongodb");
        let indexes: HashMap<(String, Vec<String>), u64> = cols
            .shop_items
            .indexes()
            .await
            .expect("Failed to populate shop_item indexes");
        for (key, entry) in indexes {
            cache.shop_item_indexes.insert(key, entry);
        }

        Ok(cache)
    }
}
