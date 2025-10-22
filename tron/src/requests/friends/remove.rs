use crate::BridgeService;
use crate::bridge::{RemoveFriendRequest, RemoveFriendResponse};
use tonic::{Request, Response, Status};
use tracing::{debug, error};

impl BridgeService {
    pub async fn handle_remove_friend(
        &self,
        request: Request<RemoveFriendRequest>,
    ) -> Result<Response<RemoveFriendResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        debug!("Remove friend request from player {} received", username);

        let player = self.cache.get_player_with_handling(&username).await?;
        let target_id = self.cache.remove_friends(&player, &target).await?;

        player
            .remove_friend(target_id, &self.databases.players)
            .await
            .map_err(|err| {
                error!(
                    "Failed to remove player {} from player {} friend list: {}",
                    target,
                    username,
                    err.to_string()
                );

                Status::internal(format!(
                    "Failed to remove player {} from your friend list",
                    target
                ))
            })?;

        Ok(Response::new(RemoveFriendResponse { success: true }))
    }
}
