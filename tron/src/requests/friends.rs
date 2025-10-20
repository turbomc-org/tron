use crate::BridgeService;
use crate::bridge::{
    AcceptFriendRequestRequest, AcceptFriendRequestResponse, GetFriendRequestsRequest,
    GetFriendRequestsResponse, GetFriendsRequest, GetFriendsResponse, RejectFriendRequestRequest,
    RejectFriendRequestResponse, SendFriendRequestRequest, SendFriendRequestResponse,
};
use crate::models::player::Player;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use tonic::{Request, Response, Status};
use tracing::error;

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
            .get_active_player(username.clone())
            .await
            .map_err(|err| {
                error!("Failed to get active player: {}", err.to_string());
                Status::internal(format!("Failed to get active player {}", username))
            })?;

        let friends = player.friends;

        Ok(Response::new(GetFriendsResponse {
            friends: friends.into_iter().collect(),
        }))
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
            .get_active_player(username.clone())
            .await
            .map_err(|err| {
                error!("Failed to get active player: {}", err.to_string());
                Status::internal(format!("Failed to get active player {}", username))
            })?;

        let friends_requests = player.incoming_friend_requests;

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
        let target = inner_request.receiver;
        let players = self.databases.players.clone();

        let projection = doc! { "_id": 1, "incoming_friend_requests": 1 };
        let find_options = FindOneOptions::builder().projection(projection).build();

        let receiver = players
            .find_one(doc! {"username": &target})
            .with_options(find_options)
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch player {} from database : {}",
                    username,
                    e.to_string()
                );

                Status::internal(format!("Failed to fetch player {} from database", username))
            })?
            .ok_or_else(|| {
                error!("Player {} not found in database", username);

                Status::not_found(format!("Player {} not found in database", username))
            })?;

        if receiver.incoming_friend_requests.contains_key(&username) {
            return Err(Status::already_exists(format!(
                "Already sent a friend request to player {}",
                &target,
            )));
        }

        Player::add_friend_request(username, target.clone(), &players)
            .await
            .map_err(|err| {
                error!(
                    "Failed to send friend request to {} due to {}",
                    target,
                    err.to_string()
                );
                Status::internal(format!("Failed to send friend request to {}", target))
            });

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
            .get_active_player(username.clone())
            .await
            .map_err(|err| {
                error!(
                    "Failed to fetch player {} from cache even though it exists: {}",
                    username, err
                );

                Status::internal(format!(
                    "Failed to fetch player {} from cache even though it exists",
                    username
                ))
            })?;

        if !player.incoming_friend_requests.contains_key(&sender) {
            error!("Player {} has no friend request from {}", sender, username);
            return Err(Status::not_found(format!(
                "Player {} has no friend request from {}",
                username, sender
            )));
        }

        Player::accept_friend_request(username.clone(), sender.clone(), &players)
            .await
            .map_err(|err| {
                error!("Failed to accept friend request from {}: {}", sender, err);

                Status::internal(format!("Failed to accept friend request from {}", sender))
            })?;

        player.incoming_friend_requests.remove(&sender);
        player.friends.insert(sender);
        self.cache
            .update_active_player(&player)
            .await
            .map_err(|err| {
                error!(
                    "Failed to update active player {} in cache: {}",
                    username, err
                );
                Status::internal(format!(
                    "Failed to update active player {} in cache",
                    username
                ))
            });

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
            .get_active_player(username.clone())
            .await
            .map_err(|err| {
                error!(
                    "Failed to fetch player {} from cache even though it exists: {}",
                    username, err
                );

                Status::internal(format!(
                    "Failed to fetch player {} from cache even though it exists",
                    username
                ))
            })?;

        if !player.incoming_friend_requests.contains_key(&sender) {
            error!("Player {} has no friend request from {}", sender, username);
            return Err(Status::not_found(format!(
                "Player {} has no friend request from {}",
                username, sender
            )));
        }

        Player::accept_friend_request(username.clone(), sender.clone(), &players)
            .await
            .map_err(|err| {
                error!("Failed to reject friend request from {}: {}", sender, err);

                Status::internal(format!("Failed to reject friend request from {}", sender))
            })?;

        player.incoming_friend_requests.remove(&sender);
        self.cache
            .update_active_player(&player)
            .await
            .map_err(|err| {
                error!(
                    "Failed to update active player {} in cache: {}",
                    username, err
                );
                Status::internal(format!(
                    "Failed to update active player {} in cache",
                    username
                ))
            });

        Ok(Response::new(RejectFriendRequestResponse { success: true }))
    }
}
