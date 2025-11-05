use crate::bridge::{ReportPlayerRequest, ReportPlayerResponse};
use crate::config::messages::REPORT_PLAYER;
use crate::models::report::Report;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_report_player(
        &self,
        request: Request<ReportPlayerRequest>,
    ) -> Result<Response<ReportPlayerResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;
        let reason = inner_request.reason;

        info!("Report player request received");

        let player = self.state.get_player_with_handling(&username).await?;
        let target_player = self.state.get_player_with_handling(&target).await?;

        let report = Report::new(player.id, target_player.id, reason);

        report
            .insert(&self.collections.reports)
            .await
            .map_err(|e| {
                error!("Failed to insert report: {}", e);
                Status::internal("Failed to insert report")
            })?;

        self.send_message_to_player(
            &username,
            render!(REPORT_PLAYER, username = target, reason = report.reason),
        )
        .await;

        info!("Report player request completed");

        Ok(Response::new(ReportPlayerResponse { success: true }))
    }
}
