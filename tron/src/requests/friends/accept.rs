use crate::config::messages::{FRIEND_CONNECTED, FRIEND_REQUEST_ACCEPTED};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};
use tron_protos::{AcceptFriendRequestRequest, AcceptFriendRequestResponse};

impl BridgeService {
    pub async fn handle_accept_friend_request(
        &self,
        request: Request<AcceptFriendRequestRequest>,
    ) -> Result<Response<AcceptFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let sender = inner_request.sender;

        info!("Accept friend request from player {} received", username);

        if username == sender {
            return Err(Status::invalid_argument(
                "Cannot accept friend request of yourself",
            ));
        }

        let mut player = self.state().get_player_with_handling(&username).await?;
        let players = &self.collections().players.clone();

        debug!(
            "Checking if {} has a friend request from {}",
            username, sender
        );

        let sender_id = self.state().check_friend_request(&player, &sender).await?;

        player
            .accept_friend_request((sender_id, sender.clone()), players, &self.state())
            .await
            .map_err(|err| {
                error!("Failed to accept friend request from {}: {}", sender, err);

                Status::internal(format!("Failed to accept friend request from {}", sender))
            })?;

        if let Err(e) = self
            .send_message(&username, render!(FRIEND_CONNECTED, sender = &sender))
            .await
        {
            error!("Failed to send player message: {}", e);
        };

        if let Err(e) = self
            .send_message(
                &sender,
                render!(FRIEND_REQUEST_ACCEPTED, username = &username),
            )
            .await
        {
            error!("Failed to send player message: {}", e);
        }

        info!("Accept friend request from player {} completed", username);

        Ok(Response::new(AcceptFriendRequestResponse { success: true }))
    }
}
