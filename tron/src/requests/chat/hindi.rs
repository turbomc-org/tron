use crate::BridgeService;
use crate::bridge::{HindiChatRequest, HindiChatResponse};
use crate::config::messages::HINDI_CHANNEL;
use crate::render;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_hindi_chat(
        &self,
        request: Request<HindiChatRequest>,
    ) -> Result<Response<HindiChatResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.state().get_player_with_handling(&username).await?;

        if self.state().messaging.is_in_global(player.id) {
            return self
                .status(
                    &username,
                    Status::already_exists("You are already in hindi chat"),
                )
                .await;
        }

        self.state().messaging.join_hindi(player.id);

        if let Err(e) = self
            .send_message(&username, render!(HINDI_CHANNEL, username = &username))
            .await
        {
            error!("Failed to send player message: {}", e);
            return Err(Status::internal(format!(
                "Failed to send hindi chat message: {}",
                e
            )));
        }

        Ok(Response::new(HindiChatResponse { success: true }))
    }
}
