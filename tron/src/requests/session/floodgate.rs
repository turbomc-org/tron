use crate::BridgeService;
use crate::bridge::{PlayerFloodgateLoginRequest, PlayerFloodgateLoginResponse};
use crate::models::player::Edition;
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    pub async fn handle_player_floodgate_login(
        &self,
        request: Request<PlayerFloodgateLoginRequest>,
    ) -> Result<Response<PlayerFloodgateLoginResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Floodgate login request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.edition != Edition::Bedrock {
            return self.status(&username, Status::internal("You are not using bedrock client but you have tried for bedrock authentication.")).await;
        }

        self.join_game(player).await;

        info!("Floodgate login request from player {} completed", username);

        Ok(Response::new(PlayerFloodgateLoginResponse {
            success: true,
        }))
    }
}
