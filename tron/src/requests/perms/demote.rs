use crate::BridgeService;
use crate::bridge::{DemotePermsRequest, DemotePermsResponse};
use crate::config::messages::DEMOTE_PERMS;
use crate::render;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_demote_perms(
        &self,
        request: Request<DemotePermsRequest>,
    ) -> Result<Response<DemotePermsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        let mut player = self.state().get_player_with_handling(&username).await?;
        let target = self.state().get_player_with_handling(&target).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admin can execute demote permission command."),
                )
                .await;
        }

        if target.is_member() {
            return self
                .status(
                    &username,
                    Status::failed_precondition("The target don't have any permission yet."),
                )
                .await;
        }

        if let Err(e) = player
            .demote(&self.collections().players, &self.state())
            .await
        {
            return self
                .status(
                    &username,
                    Status::internal(format!("Failed to demote player: {}", e)),
                )
                .await;
        }

        if let Err(e) = self
            .send_message(&username, render!(DEMOTE_PERMS, username = target.username))
            .await
        {
            error!("Failed to send player message: {}", e);
        }

        Ok(Response::new(DemotePermsResponse { success: true }))
    }
}
