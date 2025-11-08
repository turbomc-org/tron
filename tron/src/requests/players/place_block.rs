use crate::bridge::{PlayerPlaceBlockRequest, PlayerPlaceBlockResponse};
use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::{error, info};

use crate::BridgeService;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_player_place_block(
        &self,
        request: Request<PlayerPlaceBlockRequest>,
    ) -> Result<Response<PlayerPlaceBlockResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Place Block request of player {} received", username);

        let mut player = self.state().get_player_with_handling(&username).await?;

        player
            .add_blocks_placed(1, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!(
                    "Failed to add blocks placed of player {}: {}",
                    username, err
                );
                Status::internal("Failed to add death")
            })?;

        info!("Place Block request of player {} completed", username);

        Ok(Response::new(PlayerPlaceBlockResponse { success: true }))
    }
}
