use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{PlayerDeathRequest, PlayerDeathResponse};

use crate::BridgeService;

impl BridgeService {
    pub async fn handle_player_death(
        &self,
        request: Request<PlayerDeathRequest>,
    ) -> Result<Response<PlayerDeathResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let mut player = self.state().get_player_with_handling(&username).await?;

        player
            .add_death(1, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!("Failed to add death of player {}: {}", username, err);
                Status::internal("Failed to add death")
            })?;

        info!("Player {} died", username);

        Ok(Response::new(PlayerDeathResponse { success: true }))
    }
}
