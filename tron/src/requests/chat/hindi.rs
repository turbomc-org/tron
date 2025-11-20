use crate::BridgeService;
use crate::config::messages::HINDI_CHANNEL;
use crate::render;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{HindiChatRequest, HindiChatResponse};

impl BridgeService {
    pub async fn handle_hindi_chat(
        &self,
        request: Request<HindiChatRequest>,
    ) -> Result<Response<HindiChatResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Hindi chat request from player {} received", username);

        let player = self.player(&username).await?;

        if self.state().messaging.is_in_global(player.id) {
            return self
                .status(
                    &username,
                    Status::already_exists("You are already in hindi chat"),
                )
                .await;
        }

        self.state().messaging.join_hindi(player.id);

        self.send_message(&username, render!(HINDI_CHANNEL, username = &username))
            .await;

        info!("Hindi chat request from player {} completed", username);

        Ok(Response::new(HindiChatResponse { success: true }))
    }
}
