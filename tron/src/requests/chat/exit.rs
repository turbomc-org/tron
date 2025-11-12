use crate::BridgeService;
use crate::bridge::{ExitChatRequest, ExitChatResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_exit_chat(
        &self,
        request: Request<ExitChatRequest>,
    ) -> Result<Response<ExitChatResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

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

        todo!("Implement exit chat")
    }
}
