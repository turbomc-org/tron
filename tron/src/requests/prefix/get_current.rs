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
            return Err(Status::not_found("Player has no selected prefix"));
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

        Ok(Response::new(GetCurrentPrefixResponse {
            prefix: Some(prefix.compile()),
        }))
    }
}
