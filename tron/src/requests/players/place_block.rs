use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::error;
use tron_protos::{PlayerPlaceBlockRequest, PlayerPlaceBlockResponse};

use crate::BridgeService;

impl BridgeService {
    pub async fn handle_player_place_block(
        &self,
        request: Request<PlayerPlaceBlockRequest>,
    ) -> Result<Response<PlayerPlaceBlockResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let mut player = self.player(&username).await?;

        player.add_blocks_placed(1, &self).await.map_err(|err| {
            error!(
                "Failed to add blocks placed of player {}: {}",
                username, err
            );
            Status::internal("Failed to add death")
        })?;

        Ok(Response::new(PlayerPlaceBlockResponse { success: true }))
    }
}
