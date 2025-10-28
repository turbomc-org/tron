use crate::bridge::{PlayerBreakBlockRequest, PlayerBreakBlockResponse};
use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::error;

use crate::BridgeService;

impl BridgeService {
    pub async fn handle_player_break_block(
        &self,
        request: Request<PlayerBreakBlockRequest>,
    ) -> Result<Response<PlayerBreakBlockResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let mut player = self.cache.get_player_with_handling(&username).await?;

        player
            .add_blocks_broken(1, &self.collections.players, &self.cache.active_players)
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
