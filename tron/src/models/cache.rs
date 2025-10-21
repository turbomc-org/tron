use crate::models::player::Player;
use crate::models::prefix::Prefix;
use crate::models::shop_item::ShopItem;
use crate::models::team::Team;
use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;

pub struct Cache {
    pub active_players: Arc<DashMap<String, Player>>,
    pub shop_items: Arc<DashMap<u64, ShopItem>>,
    pub teams: Arc<DashMap<u64, Team>>,
    pub prefixes: Arc<DashMap<u64, Prefix>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            active_players: Arc::new(DashMap::new()),
            shop_items: Arc::new(DashMap::new()),
            teams: Arc::new(DashMap::new()),
            prefixes: Arc::new(DashMap::new()),
        }
    }

    pub async fn get_player(&self, username: &String) -> Result<Option<Player>> {
        Ok(self.active_players.get(username).map(|entry| entry.clone()))
    }

    pub async fn get_shop_item(&self, id: &u64) -> Result<Option<ShopItem>> {
        Ok(self.shop_items.get(id).map(|entry| entry.clone()))
    }

    pub async fn get_team(&self, id: &u64) -> Result<Option<Team>> {
        Ok(self.teams.get(id).map(|entry| entry.clone()))
    }

    pub async fn get_prefix(&self, id: &u64) -> Result<Option<Prefix>> {
        Ok(self.prefixes.get(id).map(|entry| entry.clone()))
    }

    pub async fn insert_player(&self, player: Player) -> Result<()> {
        self.active_players.insert(player.username.clone(), player);
        Ok(())
    }

    pub async fn insert_shop_item(&self, shop_item: ShopItem) -> Result<()> {
        self.shop_items.insert(shop_item.id, shop_item);
        Ok(())
    }

    pub async fn insert_team(&self, team: Team) -> Result<()> {
        self.teams.insert(team.id, team);
        Ok(())
    }

    pub async fn insert_prefix(&self, prefix: Prefix) -> Result<()> {
        self.prefixes.insert(prefix.id, prefix);
        Ok(())
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}
