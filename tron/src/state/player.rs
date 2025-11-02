use crate::models::achievements::Achievements;
use crate::models::player::Player;
use crate::state::State;
use anyhow::Result;
use anyhow::anyhow;
use tonic::Status;

impl State {
    pub async fn get_active_player(&self, username: &String) -> Result<Option<Player>> {
        Ok(self.active_players.get(username).map(|entry| entry.clone()))
    }

    pub async fn get_player_username(&self, id: &u64) -> Result<Option<String>> {
        Ok(self.player_indexes.get(id).map(|entry| entry.clone()))
    }

    pub async fn get_player_with_handling(&self, username: &String) -> Result<Player, Status> {
        let player = self.active_players.get(username).map(|entry| entry.clone());
        match player {
            Some(player) => Ok(player),
            None => Err(Status::not_found(format!(
                "Player {} not found or isn't active",
                username
            ))),
        }
    }

    pub async fn insert_player(&self, player: Player) -> Result<()> {
        self.active_players
            .insert(player.username.clone(), player.clone());
        self.player_indexes
            .insert(player.id.clone(), player.username.clone());

        Ok(())
    }

    pub async fn check_friend_request(&self, player: &Player, sender: &str) -> Result<u64, Status> {
        for (sender_id, _now) in player.incoming_friend_requests.iter() {
            let username_opt = self
                .get_player_username(sender_id)
                .await
                .map_err(|_| Status::not_found("Error fetching friend record"))?;

            if let Some(username) = username_opt {
                if username == sender {
                    return Ok(*sender_id);
                }
            }
        }

        Err(Status::not_found(format!(
            "You have no friend request from {}",
            sender
        )))
    }

    pub async fn get_friend_id(&self, player: &Player, target: &str) -> Result<u64, Status> {
        for friend_id in player.friends.iter() {
            let username_opt = self
                .get_player_username(friend_id)
                .await
                .map_err(|_| Status::internal("Error fetching friend username"))?;

            if let Some(username) = username_opt {
                if username == target {
                    return Ok(*friend_id);
                }
            }
        }

        Err(Status::not_found(format!(
            "Player {} is not your friend yet",
            target
        )))
    }

    pub async fn inc_coins(&self, player_id: u64, amount: i64) -> Result<()> {
        let Some(username) = self.get_player_username(&player_id).await? else {
            return Err(anyhow!("Player not found"));
        };

        let Some(mut player) = self.get_active_player(&username).await? else {
            return Err(anyhow!("Player not found"));
        };

        if amount >= 0 {
            player.coins = player.coins.saturating_add(amount as u64);
        } else {
            let sub_amount = (-amount) as u64;
            if player.coins < sub_amount {
                return Err(anyhow!("Insufficient coins"));
            }
            player.coins -= sub_amount;
        }

        self.insert_player(player.clone()).await?;
        self.update_leaderboard(player).await?;

        Ok(())
    }

    pub async fn inc_kills(&self, player_id: u64, amount: u64) -> Result<()> {
        let Some(username) = self.get_player_username(&player_id).await? else {
            return Err(anyhow!("Player not found"));
        };

        let Some(mut player) = self.get_active_player(&username).await? else {
            return Err(anyhow!("Player not found"));
        };

        player.kills = player.kills.saturating_add(amount);

        self.insert_player(player.clone()).await?;
        self.update_leaderboard(player).await?;

        Ok(())
    }

    pub async fn inc_deaths(&self, player_id: u64, amount: u64) -> Result<()> {
        let Some(username) = self.get_player_username(&player_id).await? else {
            return Err(anyhow!("Player not found"));
        };

        let Some(mut player) = self.get_active_player(&username).await? else {
            return Err(anyhow!("Player not found"));
        };

        player.deaths = player.deaths.saturating_add(amount);

        self.insert_player(player.clone()).await?;
        self.update_leaderboard(player).await?;

        Ok(())
    }

    pub async fn update_leaderboard(&self, player: Player) -> Result<()> {
        self.leaderboards
            .coins
            .update_score(player.id.clone(), player.coins.clone())
            .await;

        let overall =
            crate::utils::math::calculate_overall(player.kills, player.deaths, player.coins);

        self.leaderboards
            .overall
            .update_score(player.id.clone(), overall as u64)
            .await;

        if let Some(team) = player.team {
            let Some(team) = self.get_team(team).await? else {
                return Err(anyhow!("Team not found"));
            };

            self.update_team_score(team).await?;
        }

        Ok(())
    }

    pub async fn add_achievement(&self, player_id: u64, achievement: Achievements) -> Result<()> {
        let Some(username) = self.get_player_username(&player_id).await? else {
            return Err(anyhow!("Player not found"));
        };

        let Some(mut player) = self.get_active_player(&username).await? else {
            return Err(anyhow!("Player not found"));
        };

        player.achievements.insert(achievement);

        self.insert_player(player.clone()).await?;

        Ok(())
    }
}
