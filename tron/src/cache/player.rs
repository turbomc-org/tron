use crate::cache::Cache;
use crate::models::player::Player;
use anyhow::Result;
use futures::future::join_all;
use std::collections::{HashMap, HashSet};
use tonic::Status;
use tracing::{debug, error, info};

impl Cache {
    pub async fn get_player(&self, username: &String) -> Result<Option<Player>> {
        Ok(self.active_players.get(username).map(|entry| entry.clone()))
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
        self.active_players.insert(player.username.clone(), player);
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
        debug!(
            "Player's incoming friend requests: {:?}",
            player.incoming_friend_requests
        );
        for (sender_id, _now) in player.incoming_friend_requests.iter() {
            info!(sender_id);
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

    pub async fn get_friend_id(&self, player: &Player, target: &str) -> Result<u64, Status> {
        for friend_id in player.friends.iter() {
            if let Ok(username) = self.get_player_username(*friend_id).await {
                if username == target {
                    return Ok(*friend_id);
                }
            }
        }
        error!(
            "Player {} is not your friend of {}",
            target, player.username,
        );
        Err(Status::not_found(format!(
            "Player {} is not your friend yet",
            target
        )))
    }

    pub async fn add_friend_request(
        &self,
        receiver: String,
        sender: u64,
        now: u64,
    ) -> Result<(), Status> {
        let mut r = Self::get_player_with_handling(&self, &receiver).await?;
        r.incoming_friend_requests.insert(sender, now);
        Self::insert_player(&self, r).await.map_err(|err| {
            error!(
                "Failed to insert friend request for player {}: {}",
                receiver, err
            );
            Status::internal(format!(
                "Failed to insert friend request for player {}: {}",
                receiver, err
            ))
        })?;

        Ok(())
    }
}
