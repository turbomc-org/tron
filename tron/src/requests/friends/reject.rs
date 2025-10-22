use crate::BridgeService;
use crate::bridge::{RejectFriendRequestRequest, RejectFriendRequestResponse};
use crate::models::player::Player;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    pub async fn handle_reject_friend_request(
        &self,
        request: Request<RejectFriendRequestRequest>,
    ) -> Result<Response<RejectFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Reject friend request from player {} received", username);

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let sender = inner_request.sender;
        let players = &self.databases.players.clone();
        let sender_id = self.cache.check_friend_request(&player, &sender).await?;

        Player::reject_friend_request(player.id, sender_id, &players)
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

        info!("Reject friend request from player {} completed", username);

        Ok(Response::new(RejectFriendRequestResponse { success: true }))
    }
}
