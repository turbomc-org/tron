use crate::config::messages::{FRIEND_CHAT_REQUEST_RECEIVED, FRIEND_CHAT_REQUEST_SENT};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{FriendChatRequest, FriendChatResponse};

impl BridgeService {
    pub async fn handle_friend_chat(
        &self,
        request: Request<FriendChatRequest>,
    ) -> Result<Response<FriendChatResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let friend_username = inner_request.friend;

        info!(
            "Friend chat request from player {} to {} received",
            username, friend_username
        );

        let player = self.player(&username).await?;
        let friend = self
            .state()
            .get_player_with_handling(&friend_username)
            .await?;

        if !player.friends.contains(&friend.id) || !friend.friends.contains(&player.id) {
            return self
                .status(
                    &username,
                    Status::invalid_argument(format!(
                        "You are not friend with {}",
                        friend_username
                    )),
                )
                .await;
        }

        let token = self.state().messaging.insert_request(player.id);

        self.send_message(
            &username,
            render!(FRIEND_CHAT_REQUEST_SENT, friend = &friend_username),
        )
        .await;

        self.send_message(
            &friend_username,
            render!(
                FRIEND_CHAT_REQUEST_RECEIVED,
                friend = &username,
                token = &token
            ),
        )
        .await;

        info!(
            "Friend chat request from player {} to {} completed",
            username, friend_username
        );

        Ok(Response::new(FriendChatResponse { success: true }))
    }
}
