use crate::bridge::{PlayerKillRequest, PlayerKillResponse};
use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::{error, info};

use crate::BridgeService;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_player_kill(
        &self,
        request: Request<PlayerKillRequest>,
    ) -> Result<Response<PlayerKillResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Kill request of player {} received", username);

        let mut player = self.state().get_player_with_handling(&username).await?;

        player
            .add_kill(1, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!("Failed to add death of player {}: {}", username, err);
                Status::internal("Failed to add death")
            })?;

        info!("Kill request of player {} completed", username);

        Ok(Response::new(PlayerKillResponse { success: true }))
    }
}
