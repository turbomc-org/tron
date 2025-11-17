use crate::BridgeService;
use crate::bridge::{PlayerPostLoginRequest, PlayerPostLoginResponse};
use crate::config::messages::{RELEASE_NOTE, WELCOME_BACK};
use crate::config::release::RELEASE_CONFIG;
use crate::render;
use tonic::{Request, Response, Status};
use tracing::error;
use tracing::info;

impl BridgeService {
    pub async fn handle_player_post_login(
        &self,
        request: Request<PlayerPostLoginRequest>,
    ) -> Result<Response<PlayerPostLoginResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let proxy_id = inner_request.proxy_id;

        info!("Post login request from player {} received", username);

        if !self.state().proxy_connections.contains_key(&proxy_id) {
            error!(
                "Player {} login failed: Proxy {} has not completed its handshake.",
                &username, proxy_id
            );
            return Err(Status::unavailable(
                "Proxy server is not ready. Please reconnect.",
            ));
        }

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
