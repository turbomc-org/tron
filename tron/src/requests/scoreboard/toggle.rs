use crate::config::messages::{DISABLE_SCOREBOARD, ENABLE_SCOREBOARD};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{ToggleScoreboardRequest, ToggleScoreboardResponse};

impl BridgeService {
    pub async fn handle_toggle_scoreboard(
        &self,
        request: Request<ToggleScoreboardRequest>,
    ) -> Result<Response<ToggleScoreboardResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!(
            "Toggle scoreboard request from player {} received",
            username
        );

        let mut player = self.player(&username).await?;

        if player.scoreboard_enabled {
            if let Err(e) = player
                .set_scoreboard(false, &self.collections().players, &self.state())
                .await
            {
                error!("Failed to disable player's scoreboard: {}", e);

                return self
                    .status(
                        &username,
                        Status::internal("Failed to disable the scoreboard."),
                    )
                    .await;
            }
        } else {
            if let Err(e) = player
                .set_scoreboard(true, &self.collections().players, &self.state())
                .await
            {
                error!("Failed to enable player's scoreboard: {}", e);

                return self
                    .status(
                        &username,
                        Status::internal("Failed to enable the scoreboard."),
                    )
                    .await;
            }
        }

        if player.scoreboard_enabled {
            self.send_message(&username, render!(ENABLE_SCOREBOARD, username = &username))
                .await;
        } else {
            self.send_message(&username, render!(DISABLE_SCOREBOARD, username = &username))
                .await;
        }

        info!(
            "Toggle scoreboard request from player {} completed",
            username
        );

        Ok(Response::new(ToggleScoreboardResponse { success: true }))
    }
}
