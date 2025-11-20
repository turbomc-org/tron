use crate::config::messages::GLOBAL_CHAT;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{GlobalChatRequest, GlobalChatResponse};

impl BridgeService {
    pub async fn handle_global_chat(
        &self,
        request: Request<GlobalChatRequest>,
    ) -> Result<Response<GlobalChatResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Global chat request from player {} received", username);

        let player = self.player(&username).await?;

        if self.state().messaging.is_in_global(player.id) {
            return self
                .status(
                    &username,
                    Status::already_exists("You are already in global chat"),
                )
                .await;
        }

        self.state().messaging.join_global(player.id);

        self.send_message(&username, render!(GLOBAL_CHAT, username = &username))
            .await;

        info!("Global chat request from player {} completed", username);

        Ok(Response::new(GlobalChatResponse { success: true }))
    }
}
