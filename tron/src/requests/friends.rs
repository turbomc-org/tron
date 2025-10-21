use crate::BridgeService;
use crate::bridge::{
    AcceptFriendRequestRequest, AcceptFriendRequestResponse, GetFriendRequestsRequest,
    GetFriendRequestsResponse, GetFriendsRequest, GetFriendsResponse, RejectFriendRequestRequest,
    RejectFriendRequestResponse, SendFriendRequestRequest, SendFriendRequestResponse,
};
use crate::models::player::Player;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    pub async fn handle_get_friends(
        &self,
        request: Request<GetFriendsRequest>,
    ) -> Result<Response<GetFriendsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let players = &self.cache.active_players.clone();

        if !players.contains_key(&username) {
            error!("Player {} not found in cache", username);
            return Err(Status::not_found(format!(
                "Player {} not found in cache",
                username
            )));
        }

        let player = self
            .cache
            .get_player(&username)
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch player {} from cache: {}",
                    username,
                    e.to_string()
                );

                Status::internal(format!("Failed to fetch player {} from cache", username))
            })?
            .ok_or_else(|| {
                error!("Player {} not found in active players cache", username);

                Status::data_loss(format!(
                    "Player {} not found in active players cache",
                    username
                ))
            })?;

        let friends = self
            .cache
            .get_friend_usernames(&player)
            .await
            .map_err(|err| {
                error!(
                    "Failed to fetch friend usernames for player {}: {}",
                    username, err
                );
                Status::internal(format!(
                    "Failed to fetch friend usernames for player {}",
                    username
                ))
            })?;

        Ok(Response::new(GetFriendsResponse { friends }))
    }

    pub async fn handle_get_friend_requests(
        &self,
        request: Request<GetFriendRequestsRequest>,
    ) -> Result<Response<GetFriendRequestsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let players = &self.cache.active_players.clone();

        if !players.contains_key(&username) {
            error!("Player {} not found in cache", username);
            return Err(Status::not_found(format!(
                "Player {} not found in cache",
                username
            )));
        }

        let player = self
            .cache
            .get_player(&username)
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch player {} from cache: {}",
                    username,
                    e.to_string()
                );

                Status::internal(format!("Failed to fetch player {} from cache", username))
            })?
            .ok_or_else(|| {
                error!("Player {} not found in active players cache", username);

                Status::data_loss(format!(
                    "Player {} not found in active players cache",
                    username
                ))
            })?;

        let friends_requests = self
            .cache
            .get_friend_requests(&player)
            .await
            .map_err(|err| {
                error!(
                    "Failed to get the friend requests from index: {}",
                    err.to_string()
                );
                Status::internal(format!(
                    "Failed to get friend requests for player {}",
                    username
                ))
            })?;

        Ok(Response::new(GetFriendRequestsResponse {
            incoming_friend_requests: friends_requests.into_iter().collect(),
        }))
    }

    pub async fn handle_send_friend_request(
        &self,
        request: Request<SendFriendRequestRequest>,
    ) -> Result<Response<SendFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.sender;
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

        Player::add_friend_request(username.clone(), target.clone(), &players)
            .await
            .map_err(|err| {
                error!(
                    "Failed to send friend request to {} due to {}",
                    target,
                    err.to_string()
                );
                Status::internal(format!("Failed to send friend request to {}", target))
            })?;

        info!("Player {} send a friend request to {}", username, target);

        Ok(Response::new(SendFriendRequestResponse { success: true }))
    }

    pub async fn handle_accept_friend_request(
        &self,
        request: Request<AcceptFriendRequestRequest>,
    ) -> Result<Response<AcceptFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let sender = inner_request.sender;
        let players_cache = &self.cache.active_players.clone();
        let players = &self.databases.players.clone();

        if !players_cache.contains_key(&username) {
            error!("Player {} not found in cache", username);
            return Err(Status::not_found(format!(
                "Player {} not found in cache",
                username
            )));
        }

        let mut player = self
            .cache
            .get_player(&username)
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch player {} from cache: {}",
                    username,
                    e.to_string()
                );

                Status::internal(format!("Failed to fetch player {} from cache", username))
            })?
            .ok_or_else(|| {
                error!("Player {} not found in active players cache", username);

                Status::data_loss(format!(
                    "Player {} not found in active players cache",
                    username
                ))
            })?;

        let sender_id = self.cache.check_friend_request(&player, &sender).await?;

        Player::accept_friend_request(username.clone(), sender.clone(), &players)
            .await
            .map_err(|err| {
                error!("Failed to accept friend request from {}: {}", sender, err);

                Status::internal(format!("Failed to accept friend request from {}", sender))
            })?;

        player.incoming_friend_requests.remove(&sender_id);
        player.friends.insert(sender_id);
        self.cache.insert_player(player).await.map_err(|err| {
            error!(
                "Failed to update active player {} in cache: {}",
                username, err
            );
            Status::internal(format!(
                "Failed to update active player {} in cache",
                username
            ))
        })?;

        Ok(Response::new(AcceptFriendRequestResponse { success: true }))
    }

    pub async fn handle_reject_friend_request(
        &self,
        request: Request<RejectFriendRequestRequest>,
    ) -> Result<Response<RejectFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let sender = inner_request.sender;
        let players_cache = &self.cache.active_players.clone();
        let players = &self.databases.players.clone();

        if !players_cache.contains_key(&username) {
            error!("Player {} not found in cache", username);
            return Err(Status::not_found(format!(
                "Player {} not found in cache",
                username
            )));
        }

        let mut player = self
            .cache
            .get_player(&username)
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch player {} from cache: {}",
                    username,
                    e.to_string()
                );

                Status::internal(format!("Failed to fetch player {} from cache", username))
            })?
            .ok_or_else(|| {
                error!("Player {} not found in active players cache", username);

                Status::data_loss(format!(
                    "Player {} not found in active players cache",
                    username
                ))
            })?;

        let sender_id = self.cache.check_friend_request(&player, &sender).await?;

        Player::accept_friend_request(username.clone(), sender.clone(), &players)
            .await
            .map_err(|err| {
                error!("Failed to reject friend request from {}: {}", sender, err);

                Status::internal(format!("Failed to reject friend request from {}", sender))
            })?;

        player.incoming_friend_requests.remove(&sender_id);
        self.cache.insert_player(player).await.map_err(|err| {
            error!(
                "Failed to update active player {} in cache: {}",
                username, err
            );
            Status::internal(format!(
                "Failed to update active player {} in cache",
                username
            ))
        })?;

        Ok(Response::new(RejectFriendRequestResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use crate::BridgeService;
    use crate::bridge::bridge_server::Bridge;
    use crate::logger::Logger;

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
            sender: username,
            receiver: friend,
        });

        let friend_req_resp = service
            .send_friend_request(friend_request_req)
            .await
            .unwrap()
            .into_inner();

        assert!(friend_req_resp.success);
    }
}
