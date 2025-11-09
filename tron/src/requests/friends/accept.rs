use crate::bridge::{AcceptFriendRequestRequest, AcceptFriendRequestResponse};
use crate::config::messages::{FRIEND_CONNECTED, FRIEND_REQUEST_ACCEPTED};
use crate::models::player::Player;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
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

        Player::accept_friend_request(
            &mut player,
            (sender_id, sender.clone()),
            players,
            &self.state(),
        )
        .await
        .map_err(|err| {
            error!("Failed to accept friend request from {}: {}", sender, err);

            Status::internal(format!("Failed to accept friend request from {}", sender))
        })?;

        self.send_message(&username, render!(FRIEND_CONNECTED, sender = &sender))
            .await
            .map_err(|err| {
                error!("Failed to send player message: {}", err);
            })
            .unwrap();

        self.send_message(
            &sender,
            render!(FRIEND_REQUEST_ACCEPTED, username = &username),
        )
        .await
        .map_err(|err| {
            error!("Failed to send player message: {}", err);
        })
        .unwrap();

        info!("Accept friend request from player {} completed", username);

        Ok(Response::new(AcceptFriendRequestResponse { success: true }))
    }
}
