use crate::config::messages::FRIEND_CHAT_JOINED;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{FriendChatAcceptRequest, FriendChatAcceptResponse};

impl BridgeService {
    pub async fn handle_friend_chat_accept(
        &self,
        request: Request<FriendChatAcceptRequest>,
    ) -> Result<Response<FriendChatAcceptResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let token = inner_request.token;

        info!(
            "Friend chat accept request from player {} with token {} received",
            username, token
        );

        let player = self.player(&username).await?;

        let friend_id = match self.state().messaging.friend_chat_invites.get(&token) {
            Some(friend_r) => friend_r.value().clone(),
            None => {
                return self
                    .status(
                        &username,
                        Status::not_found("Friend chat invite not found."),
                    )
                    .await;
            }
        };

        let friend_username = match self.state().get_player_username(&friend_id) {
            Some(friend_username) => friend_username,
            None => {
                return self
                    .status(
                        &username,
                        Status::not_found("Friend's record not found in database."),
                    )
                    .await;
            }
        };

        let friend = self
            .state()
            .get_player_with_handling(&friend_username)
            .await?;

        if !player.friends.contains(&friend.id) || !friend.friends.contains(&player.id) {
            return self
                .status(
                    &username,
                    Status::invalid_argument(format!(
                        "You are no longer friends with {}.",
                        friend.username
                    )),
                )
                .await;
        }

        let stream_token = self.state().messaging.create_stream();

        self.state()
            .messaging
            .join_stream(player.id, stream_token.clone());
        self.state().messaging.join_stream(friend.id, stream_token);

        self.send_message(
            &username,
            render!(FRIEND_CHAT_JOINED, friend = &friend.username),
        )
        .await;

        self.send_message(
            &friend.username,
            render!(FRIEND_CHAT_JOINED, friend = &username),
        )
        .await;

        info!(
            "Friend chat accept request from player {} with token {} completed",
            username, token
        );

        Ok(Response::new(FriendChatAcceptResponse { success: true }))
    }
}
