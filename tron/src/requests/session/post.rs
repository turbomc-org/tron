use crate::BridgeService;
use crate::config::messages::{RELEASE_NOTE, WELCOME_BACK};
use crate::config::release::RELEASE_CONFIG;
use crate::render;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{PlayerPostLoginRequest, PlayerPostLoginResponse};

impl BridgeService {
    pub async fn handle_player_post_login(
        &self,
        request: Request<PlayerPostLoginRequest>,
    ) -> Result<Response<PlayerPostLoginResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Post login request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if let Err(e) = self
            .send_message(
                &player.username,
                render!(RELEASE_NOTE, body = &RELEASE_CONFIG.note),
            )
            .await
        {
            return Err(Status::internal(e.to_string()));
        };

        if let Err(e) = self
            .send_message(
                &player.username,
                render!(WELCOME_BACK, username = &player.username),
            )
            .await
        {
            return Err(Status::internal(e.to_string()));
        };

        info!("Post login request from player {} completed", username);

        Ok(Response::new(PlayerPostLoginResponse { success: true }))
    }
}
