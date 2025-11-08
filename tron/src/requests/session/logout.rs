use crate::BridgeService;
use crate::bridge::{PlayerLogoutRequest, PlayerLogoutResponse};
use tonic::{Request, Response, Status};
use tracing::{debug, info};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_player_logout(
        &self,
        request: Request<PlayerLogoutRequest>,
    ) -> Result<Response<PlayerLogoutResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Leave request for player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;
        self.state().active_players.remove(&player.username);

        info!("Player {} left the game", username);

        Ok(Response::new(PlayerLogoutResponse { success: true }))
    }
}
