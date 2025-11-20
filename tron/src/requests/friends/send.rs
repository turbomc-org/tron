use crate::config::messages::{FRIEND_REQUEST_SENT, NEW_FRIEND_REQUEST};
use crate::{BridgeService, render};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{SendFriendRequestRequest, SendFriendRequestResponse};

impl BridgeService {
    pub async fn handle_send_friend_request(
        &self,
        request: Request<SendFriendRequestRequest>,
    ) -> Result<Response<SendFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.sender;
        let target_username = inner_request.receiver;
        let players = self.collections().players.clone();

        info!(
            "Send friend request request from player {} received",
            username
        );

        if username == target_username {
            error!(
                "Player {} tried to send a friend request to themselves",
                username
            );
            return Err(Status::invalid_argument(
                "Cannot send friend request to yourself",
            ));
        }

        let player = self.player(&username).await?;
        let mut target = self
            .state()
            .get_player_with_handling(&target_username)
            .await?;

        let now = Utc::now().timestamp() as u64;

        player
            .add_friend_request(&mut target, now.clone(), &players, &self.state())
            .await
            .map_err(|err| {
                error!(
                    "Failed to send friend request to {} due to {}",
                    target_username,
                    err.to_string()
                );
                Status::internal(format!(
                    "Failed to send friend request to {}",
                    target_username
                ))
            })?;

        self.send_message(
            &target_username,
            render!(NEW_FRIEND_REQUEST, sender = &username),
        )
        .await;

        self.send_message(
            &player.username,
            render!(FRIEND_REQUEST_SENT, receiver = &target_username),
        )
        .await;

        info!(
            "Send friend request request from player {} completed",
            username
        );

        Ok(Response::new(SendFriendRequestResponse { success: true }))
    }
}
