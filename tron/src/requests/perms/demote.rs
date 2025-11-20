use crate::BridgeService;
use crate::config::messages::DEMOTE_PERMS;
use crate::render;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{DemotePermsRequest, DemotePermsResponse};

impl BridgeService {
    pub async fn handle_demote_perms(
        &self,
        request: Request<DemotePermsRequest>,
    ) -> Result<Response<DemotePermsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        info!("Demote perms request from player {} received", username);

        let mut player = self.player(&username).await?;
        let target = self.player(&target).await?;

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

        self.send_message(&username, render!(DEMOTE_PERMS, username = target.username))
            .await;

        info!("Demote perms request from player {} completed", username);

        Ok(Response::new(DemotePermsResponse { success: true }))
    }
}
