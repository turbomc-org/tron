use crate::models::player::Player;
use crate::state::State;
use anyhow::Result;
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
}
