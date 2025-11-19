use crate::BridgeService;
use crate::config::messages::EXIT_CHAT;
use crate::render;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{ExitChatRequest, ExitChatResponse};

impl BridgeService {
    pub async fn handle_exit_chat(
        &self,
        request: Request<ExitChatRequest>,
    ) -> Result<Response<ExitChatResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Exit chat request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if !self
            .state()
            .messaging
            .subscriptions
            .contains_key(&player.id)
        {
            return self
                .status(
                    &username,
                    Status::not_found("Join any channel first to exit it."),
                )
                .await;
        }

        self.state().messaging.exit_chat(player.id);

        if let Err(e) = self
            .send_message(&username, render!(EXIT_CHAT, username = username))
            .await
        {
            error!("Failed to send player message: {}", e);
        }

        info!("Exit chat request from player {} completed", username);

        Ok(Response::new(ExitChatResponse { success: true }))
    }
}
