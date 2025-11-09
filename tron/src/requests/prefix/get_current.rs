use crate::bridge::{GetCurrentPrefixRequest, GetCurrentPrefixResponse};
use crate::config::messages::ACTIVE_IDENTIFIER;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
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

        let player = self.state().get_player_with_handling(&username).await?;

        if player.selected_prefix.is_none() {
            return Err(Status::not_found("You have not equipped any prefix"));
        }

        let prefix = self
            .state()
            .get_prefix(&player.selected_prefix.unwrap())
            .await
            .map_err(|err| Status::internal(format!("Failed to get prefix text: {}", err)))?;

        info!(
            "Get current prefix request from player {} completed",
            username
        );

        self.send_message(
            &username,
            render!(
                ACTIVE_IDENTIFIER,
                color = &prefix.color,
                text = &prefix.text
            ),
        )
        .await
        .map_err(|err| {
            error!("Failed to send player message: {}", err);
        })
        .unwrap();

        Ok(Response::new(GetCurrentPrefixResponse {
            prefix: Some(prefix.compile()),
        }))
    }
}
