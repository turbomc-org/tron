use crate::bridge::{GlobalChatRequest, GlobalChatResponse};
use crate::config::messages::GLOBAL_CHAT;
use crate::{render, BridgeService};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_global_chat(
        &self,
        request: Request<GlobalChatRequest>,
    ) -> Result<Response<GlobalChatResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Global chat request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if self.state().messaging.is_in_global(player.id) {
            return self
                .status(
                    &username,
                    Status::already_exists("You are already in global chat"),
                )
                .await;
        }

        self.state().messaging.join_global(player.id);

        if let Err(e) = self
            .send_message(&username, render!(GLOBAL_CHAT, username = &username))
            .await
        {
            error!("Failed to send player message: {}", e);
            return Err(Status::internal(format!(
                "Failed to send global chat message: {}",
                e
            )));
        }

        info!("Global chat request from player {} completed", username);

        Ok(Response::new(GlobalChatResponse { success: true }))
    }
}
