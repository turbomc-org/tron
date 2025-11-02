use crate::BridgeService;
use crate::bridge::{AcceptFriendRequestRequest, AcceptFriendRequestResponse};
use crate::models::player::Player;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
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

        let mut player = self.state.get_player_with_handling(&username).await?;
        let players = &self.collections.players.clone();

        debug!(
            "Checking if {} has a friend request from {}",
            username, sender
        );

        let sender_id = self.state.check_friend_request(&player, &sender).await?;

        Player::accept_friend_request(
            &mut player,
            (sender_id, sender.clone()),
            players,
            &self.state,
        )
        .await
        .map_err(|err| {
            error!("Failed to accept friend request from {}: {}", sender, err);

            Status::internal(format!("Failed to accept friend request from {}", sender))
        })?;

        self.send_message_to_player(
          &username,
          format!(
            "<gradient:#C724B1:#7A00FF><bold>✅ FRIEND CONNECTED</bold></gradient>\n\
             <gray>You are now friends with <white><bold>{}</bold></white>.</gray>\n\
             <dark_gray>»</dark_gray> <click:run_command:'/friends'><u><gradient:#B200FF:#6A00A3>Open Friends List</gradient></u></click>",
            sender
          ),
        ).await;

        self.send_message_to_player(
          &sender,
          format!(
            "<gradient:#C724B1:#7A00FF><bold>⚡ FRIEND REQUEST ACCEPTED</bold></gradient>\n\
             <gray><white><bold>{}</bold></white> has accepted your connection request.</gray>\n\
             <dark_gray>»</dark_gray> <click:run_command:'/friend list'><u><gradient:#B200FF:#6A00A3>View your friends</gradient></u></click>",
            username
          ),
        ).await;

        info!("Accept friend request from player {} completed", username);

        Ok(Response::new(AcceptFriendRequestResponse { success: true }))
    }
}
