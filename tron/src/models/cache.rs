use crate::models::player::Player;
use crate::models::prefix::Prefix;
use dashmap::DashMap;
use std::sync::Arc;

use crate::models::team::Team;

pub struct Cache {
    pub active_players: Arc<DashMap<String, Vec<u8>>>,
    pub shop_items: Arc<DashMap<u64, Vec<u8>>>,
    pub teams: Arc<DashMap<u64, Vec<u8>>>,
    pub prefixes: Arc<DashMap<u64, Vec<u8>>>,
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

    pub fn get_player(&self, username: String) -> Option<Vec<u8>> {
        self.active_players.get(&username).map(|v| v.clone())
    }

    pub async fn insert_team(&self, team: Team) -> anyhow::Result<()> {
        let val = bincode::encode_to_vec(&team, bincode::config::standard())?;
        self.teams.insert(team.id, val);
        Ok(())
    }

    pub async fn get_team(&self, team_id: u64) -> anyhow::Result<Team> {
        let val = self
            .teams
            .get(&team_id)
            .map(|v| v.clone())
            .ok_or_else(|| anyhow::anyhow!("Team not found"))?;
        let team: (Team, usize) = bincode::decode_from_slice(&val, bincode::config::standard())?;
        Ok(team.0)
    }

    pub async fn get_prefix(&self, id: u64) -> anyhow::Result<Prefix> {
        let val = self
            .prefixes
            .get(&id)
            .map(|v| v.clone())
            .ok_or_else(|| anyhow::anyhow!("Prefix not found"))?;
        let prefix: (Prefix, usize) =
            bincode::decode_from_slice(&val, bincode::config::standard())?;
        Ok(prefix.0)
    }

    pub async fn insert_prefix(&self, prefix: Prefix) -> anyhow::Result<()> {
        let val = bincode::encode_to_vec(&prefix, bincode::config::standard())?;
        self.prefixes.insert(prefix.id, val);
        Ok(())
    }

    pub async fn insert_active_player(&self, player: Player) -> anyhow::Result<()> {
        let val = bincode::encode_to_vec(&player, bincode::config::standard())?;
        self.active_players.insert(player.username, val);
        Ok(())
    }

    pub async fn get_active_player(&self, player_name: String) -> anyhow::Result<Player> {
        let val = self
            .active_players
            .get(&player_name)
            .map(|v| v.clone())
            .ok_or_else(|| anyhow::anyhow!("Player not found"))?;
        let player: (Player, usize) =
            bincode::decode_from_slice(&val, bincode::config::standard())?;
        Ok(player.0)
    }

    pub async fn update_active_player(&self, player: &Player) -> anyhow::Result<()> {
        let val = bincode::encode_to_vec(player, bincode::config::standard())?;
        self.active_players.insert(player.username.clone(), val);
        Ok(())
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}
