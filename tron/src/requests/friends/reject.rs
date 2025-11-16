use crate::bridge::{RejectFriendRequestRequest, RejectFriendRequestResponse};
use crate::config::messages::{FRIEND_REQUEST_DECLINED, FRIEND_REQUEST_REJECTED};
use crate::models::player::Player;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_reject_friend_request(
        &self,
        request: Request<RejectFriendRequestRequest>,
    ) -> Result<Response<RejectFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let sender = inner_request.sender;

        info!("Reject friend request from player {} received", username);

        if username == sender {
            return Err(Status::invalid_argument(
                "Cannot reject friend request of yourself",
            ));
        }

        let mut player = self.state().get_player_with_handling(&username).await?;
        let players = &self.collections().players.clone();
        let sender_id = self.state().check_friend_request(&player, &sender).await?;

        Player::reject_friend_request(&mut player, sender_id, &players, &self.state())
            .await
            .map_err(|err| {
                error!("Failed to reject friend request from {}: {}", sender, err);

                Status::internal(format!("Failed to reject friend request from {}", sender))
            })?;

        self.send_message(
            &username,
            render!(FRIEND_REQUEST_REJECTED, sender = &sender),
        )
        .await
        .map_err(|err| {
            error!("Failed to send player message: {}", err);
        })
        .unwrap();

        self.send_message(
            &sender,
            render!(FRIEND_REQUEST_DECLINED, username = &username),
        )
        .await
        .map_err(|err| {
            error!("Failed to send player message: {}", err);
        })
        .unwrap();

        info!("Reject friend request from player {} completed", username);

        Ok(Response::new(RejectFriendRequestResponse { success: true }))
    }
}
