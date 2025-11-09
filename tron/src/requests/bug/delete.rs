use crate::BridgeService;
use crate::bridge::{DeleteBugRequest, DeleteBugResponse};
use crate::models::player::Role;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_delete_bug(
        &self,
        request: Request<DeleteBugRequest>,
    ) -> Result<Response<DeleteBugResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let bug_id = inner_request.bug_id;

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

        Ok(Response::new(DeleteBugResponse { success: true }))
    }
}
