use crate::BridgeService;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{PlayerLogoutRequest, PlayerLogoutResponse};

impl BridgeService {
    pub async fn handle_player_logout(
        &self,
        request: Request<PlayerLogoutRequest>,
    ) -> Result<Response<PlayerLogoutResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.player(&username).await?;
        self.state().active_players.remove(&player.username);

        info!("Player {} left the game", username);

        Ok(Response::new(PlayerLogoutResponse { success: true }))
    }
}
