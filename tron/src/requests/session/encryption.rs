use crate::BridgeService;
use crate::bridge::PlayerEncryptionLoginRequest;
use crate::bridge::PlayerEncryptionLoginResponse;
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    pub async fn handle_player_encryption_login(
        &self,
        request: Request<PlayerEncryptionLoginRequest>,
    ) -> Result<Response<PlayerEncryptionLoginResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Encryption login request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.alias.is_none() {
            return self.status(&username, Status::internal("You are not a player using encryption authentication but still you tried for it.")).await;
        }

        self.join_game(player).await;

        info!(
            "Encryption login request from player {} completed",
            username
        );

        Ok(Response::new(PlayerEncryptionLoginResponse {
            success: true,
        }))
    }
}
