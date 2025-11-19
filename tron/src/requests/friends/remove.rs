use crate::config::messages::FRIEND_REMOVED;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{RemoveFriendRequest, RemoveFriendResponse};

impl BridgeService {
    pub async fn handle_remove_friend(
        &self,
        request: Request<RemoveFriendRequest>,
    ) -> Result<Response<RemoveFriendResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        info!("Remove friend request from player {} received", username);

        if username == target {
            return Err(Status::invalid_argument(
                "Cannot remove yourself from your friend list",
            ));
        }

        let mut player = self.state().get_player_with_handling(&username).await?;
        let target_id = self.state().get_friend_id(&player, &target).await?;

        player
            .remove_friend(target_id, &self.collections().players, &self.state())
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

        self.send_message(&username, render!(FRIEND_REMOVED, target = &target))
            .await
            .map_err(|err| {
                error!("Failed to send player message: {}", err);
            })
            .unwrap();

        info!("Remove friend request from player {} completed", username);

        Ok(Response::new(RemoveFriendResponse { success: true }))
    }
}
