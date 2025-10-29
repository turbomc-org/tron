use crate::BridgeService;
use crate::bridge::{GetCurrentPrefixRequest, GetCurrentPrefixResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_current_prefix(
        &self,
        request: Request<GetCurrentPrefixRequest>,
    ) -> Result<Response<GetCurrentPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.state.get_player_with_handling(&username).await?;

        if player.selected_prefix.is_none() {
            return Err(Status::not_found("Player has no selected prefix"));
        }

        let prefix = self
            .state
            .get_prefix(&player.selected_prefix.unwrap())
            .await
            .map_err(|err| Status::internal(format!("Failed to get prefix text: {}", err)))?;

        Ok(Response::new(GetCurrentPrefixResponse {
            prefix: Some(prefix.compile()),
        }))
    }
}
