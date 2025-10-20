use crate::models::player::Player;
use dashmap::DashMap;
use std::sync::Arc;

use crate::models::team::Team;

pub struct Cache {
    pub active_players: Arc<DashMap<String, Vec<u8>>>,
    pub shop_items: Arc<DashMap<String, Vec<u8>>>,
    pub teams: Arc<DashMap<String, Vec<u8>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            active_players: Arc::new(DashMap::new()),
            shop_items: Arc::new(DashMap::new()),
            teams: Arc::new(DashMap::new()),
        }
    }

    pub fn get_player(&self, username: String) -> Option<Vec<u8>> {
        self.active_players.get(&username).map(|v| v.clone())
    }

    pub async fn insert_team(&self, team: Team) -> anyhow::Result<()> {
        let val = bincode::encode_to_vec(&team, bincode::config::standard())?;
        self.teams.insert(team.name, val);
        Ok(())
    }

    pub async fn get_team(&self, team_name: String) -> anyhow::Result<Team> {
        let val = self
            .teams
            .get(&team_name)
            .map(|v| v.clone())
            .ok_or_else(|| anyhow::anyhow!("Team not found"))?;
        let team: (Team, usize) = bincode::decode_from_slice(&val, bincode::config::standard())?;
        Ok(team.0)
    }

    pub async fn insert_active_player(&self, player: Player) -> anyhow::Result<()> {
        let val = bincode::encode_to_vec(&player, bincode::config::standard())?;
        self.active_players.insert(player.player_name, val);
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
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}
