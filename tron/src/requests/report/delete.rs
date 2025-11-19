use crate::BridgeService;
use crate::models::player::Role;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{DeleteReportRequest, DeleteReportResponse};

impl BridgeService {
    pub async fn handle_delete_report(
        &self,
        request: Request<DeleteReportRequest>,
    ) -> Result<Response<DeleteReportResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let report_id = inner_request.id;

        info!("Delete report request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use delete bug command."),
                )
                .await;
        }

        if let Err(e) = self.collections().reports.delete_one(report_id).await {
            error!("Failed to delete report from database: {}", e);

            return self
                .status(
                    &username,
                    Status::internal("Failed to delete report from database."),
                )
                .await;
        }

        info!("Delete report request from player {} completed", username);

        Ok(Response::new(DeleteReportResponse { success: true }))
    }
}
