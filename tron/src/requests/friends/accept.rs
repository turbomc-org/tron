use crate::BridgeService;
use crate::bridge::{AcceptFriendRequestRequest, AcceptFriendRequestResponse};
use crate::models::player::Player;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    pub async fn handle_accept_friend_request(
        &self,
        request: Request<AcceptFriendRequestRequest>,
    ) -> Result<Response<AcceptFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Accept friend request from {} received", username);

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let sender = inner_request.sender;
        let players = &self.databases.players.clone();

        debug!(
            "Checking if {} has a friend request from {}",
            username, sender
        );

        let sender_id = self.cache.check_friend_request(&player, &sender).await?;

        Player::accept_friend_request(
            &mut player,
            (sender_id, sender.clone()),
            players,
            &self.cache.active_players,
        )
        .await
        .map_err(|err| {
            error!("Failed to accept friend request from {}: {}", sender, err);

            Status::internal(format!("Failed to accept friend request from {}", sender))
        })?;

        info!("Accept friend request from {} completed", username);

        Ok(Response::new(AcceptFriendRequestResponse { success: true }))
    }
}
