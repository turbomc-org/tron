use crate::config::messages::DELETE_BUG;
use crate::models::player::Role;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{DeleteBugRequest, DeleteBugResponse};

impl BridgeService {
    pub async fn handle_delete_bug(
        &self,
        request: Request<DeleteBugRequest>,
    ) -> Result<Response<DeleteBugResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let bug_id = inner_request.bug_id;

        info!(
            "Delete bug request from player {} for bug {} received",
            username, bug_id
        );

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use delete bug command."),
                )
                .await;
        }

        if let Err(e) = self.collections().bugs.delete_one(bug_id).await {
            error!("Failed to delete bug from database: {}", e);

            return self
                .status(
                    &username,
                    Status::internal("Failed to delete bug from database."),
                )
                .await;
        }

        if let Err(e) = self
            .send_message(&username, render!(DELETE_BUG, username = username))
            .await
        {
            error!("Failed to send player message: {}", e)
        }

        info!(
            "Delete bug request from player {} for bug {} completed",
            username, bug_id
        );

        Ok(Response::new(DeleteBugResponse { success: true }))
    }
}
