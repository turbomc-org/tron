use crate::BridgeService;
use crate::bridge::{GetCurrentPrefixRequest, GetCurrentPrefixResponse};
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_get_current_prefix(
        &self,
        request: Request<GetCurrentPrefixRequest>,
    ) -> Result<Response<GetCurrentPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!(
            "Get current prefix request from player {} received",
            username
        );

        let player = self.state.get_player_with_handling(&username).await?;

        if player.selected_prefix.is_none() {
            return Err(Status::not_found("You have not equipped any prefix"));
        }

        let prefix = self
            .state
            .get_prefix(&player.selected_prefix.unwrap())
            .await
            .map_err(|err| Status::internal(format!("Failed to get prefix text: {}", err)))?;

        info!(
            "Get current prefix request from player {} completed",
            username
        );

        self.send_message_to_player(
          &username,
          format!(
            "<gradient:#C724B1:#7A00FF><bold>ℹ️ ACTIVE IDENTIFIER</bold></gradient>\n\
             <gray>Your currently equipped network identifier is <color:{}>{}</color>.</gray>\n\
             <dark_gray>»</dark_gray> <click:run_command:'/prefix unequip'><u><gradient:#B200FF:#6A00A3>Click to unequip</gradient></u></click>",
             prefix.color, prefix.text
          ),
        ).await;

        Ok(Response::new(GetCurrentPrefixResponse {
            prefix: Some(prefix.compile()),
        }))
    }
}
