use crate::BridgeService;
use crate::bridge::{SendFriendRequestRequest, SendFriendRequestResponse};
use crate::models::player::Player;
use chrono::Utc;
use mongodb::options::FindOneOptions;
use mongodb::{Collection, bson::doc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    pub async fn handle_send_friend_request(
        &self,
        request: Request<SendFriendRequestRequest>,
    ) -> Result<Response<SendFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.sender;
        let player = self.cache.get_player_with_handling(&username).await?;
        let id = self
            .cache
            .get_active_player_id(&username)
            .await
            .map_err(|err| {
                error!("Failed to get active player ID for {}: {}", username, err);
                Status::data_loss(format!(
                    "Failed to get active player ID for {}: {}",
                    username, err
                ))
            })?;
        let target = inner_request.receiver;
        let players = self.databases.players.clone();

        debug!(
            "Friend request for player {} received from {}",
            target, username
        );

        #[derive(Serialize, Deserialize)]
        struct PartialResponse {
            #[serde(rename = "_id")]
            id: u64,
            #[serde(
                serialize_with = "crate::utils::serde::serialize_u64_map",
                deserialize_with = "crate::utils::serde::deserialize_u64_map"
            )]
            incoming_friend_requests: HashMap<u64, u64>,
        }

        let partial_players: Collection<PartialResponse> = self.databases.players.clone_with_type();
        let projection = doc! { "_id": 1, "incoming_friend_requests": 1  };
        let find_options = FindOneOptions::builder().projection(projection).build();

        debug!("Fetching player {} from database", target);

        let receiver = partial_players
            .find_one(doc! {"username": &target})
            .with_options(find_options)
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch player {} from database : {}",
                    target,
                    e.to_string()
                );

                Status::internal(format!("Failed to fetch player {} from database", username))
            })?
            .ok_or_else(|| {
                error!("Player {} not found in database", username);

                Status::not_found(format!("Player {} not found in database", username))
            })?;

        debug!(
            "Fetched player {} from database with id {}",
            target, receiver.id
        );

        if receiver.incoming_friend_requests.contains_key(&id) {
            error!(
                "Player {} already sent a friend request to {}",
                username, target
            );

            return Err(Status::already_exists(format!(
                "Already sent a friend request to player {}",
                &target,
            )));
        }

        let now = Utc::now().timestamp() as u64;

        Player::add_friend_request(player.id, receiver.id, now.clone(), &players)
            .await
            .map_err(|err| {
                error!(
                    "Failed to send friend request to {} due to {}",
                    target,
                    err.to_string()
                );
                Status::internal(format!("Failed to send friend request to {}", target))
            })?;

        debug!("Checking if player {} is online", target);
        if self.cache.active_players.contains_key(&target) {
            debug!("Adding request to player {}", target);
            self.cache
                .add_friend_request(target.clone(), player.id, now)
                .await?;
        }

        info!("Player {} send a friend request to {}", username, target);

        Ok(Response::new(SendFriendRequestResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use crate::BridgeService;
    use crate::bridge::bridge_server::Bridge;
    use crate::logger::Logger;
    use mongodb::bson::doc;

    #[tokio::test]
    async fn test_send_friend_request() {
        Logger::init(true).await;

        let service = BridgeService::new().await;

        let username = "vaibhav_57890".to_string();
        let friend = "biharini_57809".to_string();
        let edition = 1;

        let player_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
            username: username.clone(),
            edition,
        });

        let player_resp = service.player_join(player_req).await.unwrap().into_inner();

        assert!(player_resp.success);

        let friend_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
            username: friend.clone(),
            edition,
        });

        let friend_resp = service.player_join(friend_req).await.unwrap().into_inner();

        assert!(friend_resp.success);

        let friend_request_req = tonic::Request::new(crate::bridge::SendFriendRequestRequest {
            sender: username.clone(),
            receiver: friend.clone(),
        });

        let friend_req_resp = service
            .send_friend_request(friend_request_req)
            .await
            .unwrap()
            .into_inner();

        assert!(friend_req_resp.success);

        let sender_document = service.cache.get_player(&username).await.unwrap().unwrap();
        let receiver_document = service.cache.get_player(&friend).await.unwrap().unwrap();

        let verification = receiver_document
            .incoming_friend_requests
            .contains_key(&sender_document.id);

        assert!(verification);

        service
            .databases
            .players
            .delete_one(doc! {"username": username})
            .await
            .unwrap();

        service
            .databases
            .players
            .delete_one(doc! {"username": friend})
            .await
            .unwrap();
    }
}
