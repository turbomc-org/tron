use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::error;
use tron_protos::{PlayerBreakBlockRequest, PlayerBreakBlockResponse};

use crate::BridgeService;

impl BridgeService {
    pub async fn handle_player_break_block(
        &self,
        request: Request<PlayerBreakBlockRequest>,
    ) -> Result<Response<PlayerBreakBlockResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let mut player = self.player(&username).await?;

        player
            .add_blocks_broken(1, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!(
                    "Failed to add blocks placed of player {}: {}",
                    username, err
                );
                Status::internal("Failed to add death")
            })?;

        Ok(Response::new(PlayerBreakBlockResponse { success: true }))
    }
}
