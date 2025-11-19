pub mod delete;
pub mod get;
pub mod get_all;
pub mod list_all;
pub mod view;

use crate::BridgeService;
use tron_protos::{BugRequest, BugResponse};
use crate::config::messages::{BUG_SUBMITTED, PLAYER_BUG_NOTIFICATION};
use crate::models::bug::Bug;
use crate::render;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_bug(
        &self,
        request: Request<BugRequest>,
    ) -> Result<Response<BugResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let description = inner_request.description;

        let player = self.state().get_player_with_handling(&username).await?;

        let bug = Bug::new(player.id, description);

        if let Err(e) = bug.insert(&self.collections().bugs).await {
            error!("Failed to register bug: {}", e);
            return self
                .status(&username, Status::internal("Failed to register bug."))
                .await;
        }

        if let Err(e) = self
            .send_message(&username, render!(BUG_SUBMITTED, username = username))
            .await
        {
            error!("Failed to send player message: {}", e);
        }

        if let Err(e) = self
            .send_message_to_admins(render!(PLAYER_BUG_NOTIFICATION, username = username))
            .await
        {
            error!("Failed to send player message: {}", e);
        }

        if let Err(e) = self
            .send_message_to_moderators(render!(PLAYER_BUG_NOTIFICATION, username = username))
            .await
        {
            error!("Failed to send player message: {}", e);
        }

        Ok(Response::new(BugResponse { success: true }))
    }
}
