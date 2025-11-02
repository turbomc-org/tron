use crate::BridgeService;
use crate::bridge::{RejectFriendRequestRequest, RejectFriendRequestResponse};
use crate::models::player::Player;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
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

        let mut player = self.state.get_player_with_handling(&username).await?;
        let players = &self.collections.players.clone();
        let sender_id = self.state.check_friend_request(&player, &sender).await?;

        Player::reject_friend_request(&mut player, sender_id, &players, &self.state)
            .await
            .map_err(|err| {
                error!("Failed to reject friend request from {}: {}", sender, err);

                Status::internal(format!("Failed to reject friend request from {}", sender))
            })?;

        self.send_message_to_player(
            &username,
            format!(
                "<gradient:#FF4D4D:#FF0000><bold>❌ FRIEND REQUEST REJECTED</bold></gradient>\n\
                 <gray>You have rejected the friend request from <white><bold>{}</bold></white>.</gray>\n\
                 <dark_gray>»</dark_gray> <click:run_command:'/friends'><u><gradient:#FF4D4D:#FF0000>View pending requests</gradient></u></click>",
                sender
            ),
        ).await;

        self.send_message_to_player(
                &sender,
                format!(
                    "<gradient:#FF4D4D:#FF0000><bold>❌ FRIEND REQUEST DECLINED</bold></gradient>\n\
                     <gray>Your friend request to <white><bold>{}</bold></white> was rejected.</gray>",
                    username
                ),
            ).await;

        info!("Reject friend request from player {} completed", username);

        Ok(Response::new(RejectFriendRequestResponse { success: true }))
    }
}
