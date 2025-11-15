use crate::bridge::ProxyConnectionResponse;
use crate::collections::Collections;
use crate::models::leaderboards::Leaderboards;
use crate::models::player::Player;
use crate::models::prefix::Prefix;
use crate::models::redeem::Redeem;
use crate::models::servers::Servers;
use crate::models::shop_item::ShopItem;
use crate::models::team::Team;
use crate::modules::indexes::Indexes;
use crate::state::messaging::Messaging;
use crate::state::permissions::Permissions;
use dashmap::{DashMap, DashSet};
use std::collections::{HashMap, HashSet};
use tokio::sync::mpsc;
use tonic::Status;
use tracing::info;

pub mod auth;
pub mod messaging;
pub mod permissions;
pub mod player;
pub mod prefix;
pub mod redeem;
pub mod server;
pub mod shop_item;
pub mod team;

#[derive(Debug)]
pub struct State {
    pub aliases: DashMap<String, String>,
    pub permissions: Permissions,
    pub active_players: DashMap<String, Player>,
    pub shop_items: DashMap<u64, ShopItem>,
    pub teams: DashMap<u64, Team>,
    pub redeems: DashMap<u64, Redeem>,
    pub prefixes: DashMap<u64, Prefix>,
    pub leaderboards: Leaderboards,
    pub servers: Servers,
    pub indexes: Indexes,
    pub proxies: DashSet<u64>,
    pub proxy_connections: DashMap<u64, mpsc::Sender<Result<ProxyConnectionResponse, Status>>>,
    pub messaging: Messaging,
}

impl State {
    pub fn new() -> Self {
        Self {
            aliases: DashMap::new(),
            active_players: DashMap::new(),
            shop_items: DashMap::new(),
            permissions: Permissions::new(),
            teams: DashMap::new(),
            prefixes: DashMap::new(),
            leaderboards: Leaderboards::new(),
            servers: Servers::new(),
            indexes: Indexes::new(),
            proxy_connections: DashMap::new(),
            messaging: Messaging::new(),
            redeems: DashMap::new(),
        }
    }

    pub async fn init(cols: &Collections) -> anyhow::Result<Self> {
        let cache = Self::new();

        info!("Populating shop items to cache from database");
        let shop_items: Vec<ShopItem> = cols.shop_items.all().await?;
        for item in shop_items {
            cache.shop_items.insert(item.id, item);
        }

        info!("Populating teams to cache from database");
        let teams: Vec<Team> = cols.teams.all().await?;
        for team in &teams {
            cache.teams.insert(team.id, team.clone());
        }

        info!("Populating prefixes to cache from database");
        let prefixes: Vec<Prefix> = cols.prefixes.all().await?;
        for prefix in prefixes {
            cache.prefixes.insert(prefix.id, prefix);
        }

        info!("Populating players indexes from database");
        let indexes: HashMap<u64, String> = cols
            .players
            .indexes()
            .await
            .expect("Failed to populate player indexes");
        for (id, username) in indexes {
            cache.indexes.player.insert(id, username);
        }

        info!("Populating teams indexes from database");
        let indexes: HashMap<String, u64> = cols
            .teams
            .indexes()
            .await
            .expect("Failed to populate team indexes");
        for (name, id) in indexes {
            cache.indexes.team.insert(name, id);
        }

        info!("Populating open teams indexes from database");
        let indexes: HashSet<u64> = cols
            .teams
            .open_indexes()
            .await
            .expect("Failed to populate open team indexes");
        for id in indexes {
            cache.indexes.open_team.insert(id);
        }

        info!("Populating prefix indexes from database");
        let indexes: HashMap<String, u64> = cols
            .prefixes
            .indexes()
            .await
            .expect("Failed to populate prefix indexes");
        for (text, id) in indexes {
            cache.indexes.prefix.insert(text, id);
        }

        info!("Populating shop_item indexes from database");
        let indexes: HashMap<(String, Vec<String>), u64> = cols
            .shop_items
            .indexes()
            .await
            .expect("Failed to populate shop_item indexes");
        for (key, entry) in indexes {
            cache.indexes.shop_item.insert(key, entry);
        }

        info!("Populating kills leaderboard from database");
        let kills_indexes: HashMap<u64, u64> = cols
            .players
            .kill_indexes()
            .await
            .expect("Failed to populate kills leaderboard");

        for (id, kills) in &kills_indexes {
            cache.leaderboards.kills.update_score(*id, *kills).await
        }

        info!("Populating deaths leaderboard from database");
        let deaths_indexes: HashMap<u64, u64> = cols
            .players
            .death_indexes()
            .await
            .expect("Failed to populate deaths leaderboard");

        for (id, deaths) in &deaths_indexes {
            cache.leaderboards.deaths.update_score(*id, *deaths).await
        }

        info!("Populating coins leaderboard from database");
        let coins_indexes: HashMap<u64, u64> = cols
            .players
            .coin_indexes()
            .await
            .expect("Failed to populate coins leaderboard");

        for (id, coins) in &coins_indexes {
            cache.leaderboards.coins.update_score(*id, *coins).await
        }

        info!("Populating kd leaderboard from database");
        let mut kd_indexes: HashMap<u64, f64> = HashMap::new();

        for entry in kills_indexes.iter() {
            let id = *entry.0;
            let kills_val = *entry.1;
            let deaths_val = *deaths_indexes.get(&id).unwrap_or(&0);

            let kd = crate::utils::math::calculate_kd(kills_val, deaths_val);
            kd_indexes.insert(id, kd);
        }

        for (id, score) in kd_indexes {
            cache.leaderboards.kd.update_score(id, score as u64).await
        }

        info!("Populating overall leaderboard from database");
        let mut overall_indexes: HashMap<u64, f64> = HashMap::new();

        for entry in kills_indexes.iter() {
            let id = *entry.0;
            let kills_val = *entry.1;
            let deaths_val = *deaths_indexes.get(&id).unwrap_or(&0);
            let coins_val = *coins_indexes.get(&id).unwrap_or(&0);

            let overall = crate::utils::math::calculate_overall(kills_val, deaths_val, coins_val);
            overall_indexes.insert(id, overall);
        }

        for (id, score) in &overall_indexes {
            cache
                .leaderboards
                .overall
                .update_score(*id, *score as u64)
                .await
        }

        info!("Populating team leaderboard from database");
        let mut team_indexes: HashMap<u64, f64> = HashMap::new();

        for team in &teams {
            let mut team_score = 0.0;
            for member in &team.members {
                let member_id = member.0;
                let overall_score = overall_indexes.get(&member_id).unwrap_or(&0.0);
                team_score += overall_score;
            }

            team_score = team_score / 2 as f64;
            team_indexes.insert(team.id, team_score);
        }

        for (id, score) in team_indexes {
            cache
                .leaderboards
                .teams
                .update_score(id, score as u64)
                .await
        }

        info!("Setting up teams messaging streams");

        for team in teams {
            cache.messaging.register_team_stream(team.id);
        }

        info!("Populating aliases from database");

        Ok(cache)
    }
}
