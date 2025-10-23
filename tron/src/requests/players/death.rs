use crate::bridge::{PlayerDeathRequest, PlayerDeathResponse};
use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::error;

use crate::BridgeService;

impl BridgeService {
    pub async fn handle_player_death(
        &self,
        request: Request<PlayerDeathRequest>,
    ) -> Result<Response<PlayerDeathResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let mut player = self.cache.get_player_with_handling(&username).await?;

        player
            .add_death(1, &self.databases.players, &self.cache.active_players)
            .await
            .map_err(|err| {
                error!("Failed to add death of player {}: {}", username, err);
                Status::internal("Failed to add death")
            })?;

        Ok(Response::new(PlayerDeathResponse { success: true }))
    }
}
