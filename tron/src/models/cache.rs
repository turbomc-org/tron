use crate::models::player::Player;
use crate::models::prefix::Prefix;
use crate::models::shop_item::ShopItem;
use crate::models::team::Team;
use anyhow::Result;
use dashmap::DashMap;
use futures::future::join_all;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tonic::Status;
use tracing::error;

pub struct Cache {
    pub active_players: Arc<DashMap<String, Player>>,
    pub shop_items: Arc<DashMap<u64, ShopItem>>,
    pub teams: Arc<DashMap<u64, Team>>,
    pub prefixes: Arc<DashMap<u64, Prefix>>,
    pub player_indexes: Arc<DashMap<u64, String>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            active_players: Arc::new(DashMap::new()),
            shop_items: Arc::new(DashMap::new()),
            teams: Arc::new(DashMap::new()),
            prefixes: Arc::new(DashMap::new()),
            player_indexes: Arc::new(DashMap::new()),
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

    pub async fn get_active_player_id(&self, username: &String) -> Result<u64> {
        Ok(self
            .active_players
            .get(username)
            .map(|player| player.id)
            .unwrap())
    }

    pub async fn get_player_username(&self, id: u64) -> Result<String> {
        Ok(self
            .player_indexes
            .get(&id)
            .map(|entry| entry.clone())
            .unwrap())
    }

    pub async fn get_friend_usernames(&self, player: &Player) -> Result<Vec<String>> {
        let friend_ids: HashSet<u64> = player.friends.clone();
        let username_futures = friend_ids
            .into_iter()
            .map(|id| self.get_player_username(id));
        let usernames = join_all(username_futures).await;
        usernames.into_iter().collect::<Result<Vec<String>, _>>()
    }

    pub async fn get_friend_requests(&self, player: &Player) -> Result<HashMap<String, u64>> {
        let friend_requests = player.incoming_friend_requests.clone();

        let username_futures = friend_requests.into_iter().map(|(id, time)| async move {
            let username = self.get_player_username(id).await?;
            Ok((username, time))
        });

        let results: Vec<Result<(String, u64)>> = join_all(username_futures).await;

        let map = results
            .into_iter()
            .collect::<Result<HashMap<String, u64>, _>>()?;

        Ok(map)
    }

    pub async fn check_friend_request(&self, player: &Player, sender: &str) -> Result<u64, Status> {
        for (_request_id, sender_id) in player.incoming_friend_requests.iter() {
            if let Ok(username) = self.get_player_username(*sender_id).await {
                if username == sender {
                    return Ok(*sender_id);
                }
            }
        }
        error!(
            "Player {} has no friend request from {}",
            player.username, sender
        );
        Err(Status::not_found(format!(
            "Player {} has no friend request from {}",
            player.username, sender
        )))
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{
        GENERATOR,
        models::{
            player::Player,
            shop_item::{ItemType, Rarity},
        },
    };

    #[tokio::test]
    async fn test_insert_and_get_player_cache() {
        let cache = Cache::new();
        let player = Player {
            username: "Hari".to_string(),
            coins: 100,
            ..Player::new("Hari".to_string(), crate::models::player::Edition::Java)
        };

        cache.insert_player(player.clone()).await.unwrap();

        let found = cache.get_player(&"Hari".to_string()).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().coins, 100);
    }

    #[tokio::test]
    async fn test_insert_and_update_player_cache() {
        let cache = Cache::new();
        let mut player = Player {
            username: "harihar".to_string(),
            coins: 100,
            ..Player::new("harihar".to_string(), crate::models::player::Edition::Java)
        };

        cache.insert_player(player.clone()).await.unwrap();

        player.coins = 150;

        cache.insert_player(player).await.unwrap();

        let found = cache
            .get_player(&"harihar".to_string())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(found.coins, 150)
    }

    #[tokio::test]
    async fn test_insert_and_get_shop_item_cache() {
        let cache = Cache::new();
        let shop_item = ShopItem::new(
            "DIAMOND_SWORD".to_string(),
            ItemType::Tool,
            "Diamond Sword".to_string(),
            "long live draft punk".to_string(),
            "weapon".to_string(),
            HashSet::new(),
            Rarity::Uncommon,
            100,
            80,
        );

        cache.insert_shop_item(shop_item.clone()).await.unwrap();

        let found = cache.get_shop_item(&shop_item.id).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, shop_item.id);
    }

    #[tokio::test]
    async fn test_insert_and_update_shop_item_cache() {
        let cache = Cache::new();
        let mut shop_item = ShopItem::new(
            "DIAMOND_SWORD".to_string(),
            ItemType::Tool,
            "Diamond Sword".to_string(),
            "long live draft punk".to_string(),
            "weapon".to_string(),
            HashSet::new(),
            Rarity::Uncommon,
            100,
            80,
        );

        cache.insert_shop_item(shop_item.clone()).await.unwrap();

        shop_item.item.id = "WOODEN_SWORD".to_string();

        cache.insert_shop_item(shop_item.clone()).await.unwrap();
        let found = cache.get_shop_item(&shop_item.id).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().item.id, "WOODEN_SWORD".to_string());
    }

    #[tokio::test]
    async fn test_insert_and_get_prefix_cache() {
        let cache = Cache::new();
        let prefix = Prefix::new("BIHARINO1".to_string(), "#000000".to_string(), 100_000);

        cache.insert_prefix(prefix.clone()).await.unwrap();

        let found = cache.get_prefix(&prefix.id).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, prefix.id);
    }

    #[tokio::test]
    async fn test_insert_and_update_prefix_cache() {
        let cache = Cache::new();
        let mut prefix = Prefix::new("BIHARINO1".to_string(), "#000000".to_string(), 100_000);

        cache.insert_prefix(prefix.clone()).await.unwrap();

        let found = cache.get_prefix(&prefix.id).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, prefix.id);

        prefix.text = "BIHARINO2".to_string();
        prefix.display_color_hex = "#FFFFFF".to_string();
        prefix.price = 200_000;

        cache.insert_prefix(prefix.clone()).await.unwrap();

        let found = cache.get_prefix(&prefix.id).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.clone().unwrap().text, "BIHARINO2".to_string());
        assert_eq!(
            found.clone().unwrap().display_color_hex,
            "#FFFFFF".to_string()
        );
        assert_eq!(found.unwrap().price, 200_000);
    }

    #[tokio::test]
    async fn test_insert_and_get_team_cache() {
        let cache = Cache::new();
        let leader: u64 = GENERATOR.generate();
        let team = Team::new(
            "bihari".to_string(),
            leader.clone(),
            false,
            "#FFFFFF".to_string(),
        );

        cache.insert_team(team.clone()).await.unwrap();

        let found = cache.get_team(&team.id).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, team.id);
    }

    #[tokio::test]
    async fn test_insert_and_update_team_cache() {
        let cache = Cache::new();
        let leader: u64 = GENERATOR.generate();
        let mut team = Team::new(
            "bihari".to_string(),
            leader.clone(),
            false,
            "#FFFFFF".to_string(),
        );

        cache.insert_team(team.clone()).await.unwrap();

        team.name = "BIHARI2".to_string();

        cache.insert_team(team.clone()).await.unwrap();

        let found = cache.get_team(&team.id).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "BIHARI2".to_string());
    }
}
