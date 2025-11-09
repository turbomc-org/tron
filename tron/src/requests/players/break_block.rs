use crate::bridge::{PlayerBreakBlockRequest, PlayerBreakBlockResponse};
use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::{error, info};

use crate::BridgeService;

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_player_break_block(
        &self,
        request: Request<PlayerBreakBlockRequest>,
    ) -> Result<Response<PlayerBreakBlockResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Break block request of player {} received", username);

        let mut player = self.state().get_player_with_handling(&username).await?;

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

        info!("Break block request of player {} completed", username);

        Ok(Response::new(PlayerBreakBlockResponse { success: true }))
    }
}
