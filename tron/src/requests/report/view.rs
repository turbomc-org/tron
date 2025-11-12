use crate::BridgeService;
use crate::bridge::{ViewReportRequest, ViewReportResponse};
use crate::config::messages::REPORT_DETAIL;
use crate::models::player::Role;
use crate::render;
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_view_report(
        &self,
        request: Request<ViewReportRequest>,
    ) -> Result<Response<ViewReportResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let report_id = inner_request.id;

        info!("View report {} requested by {}", report_id, username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can view report."),
                )
                .await;
        }

        let report = match self.collections().reports.find_by_id(report_id).await {
            Ok(report) => report,
            Err(err) => {
                error!("Failed to fetch report {}: {}", report_id, err);
                return self
                    .status(
                        &username,
                        Status::internal("Failed to fetch report from database."),
                    )
                    .await;
            }
        };

        let Some(report) = report else {
            return self
                .status(
                    &username,
                    Status::not_found("Failed to fetch report from database."),
                )
                .await;
        };

        let reporter = match self.state().get_player_username(&report.player_id) {
            Some(name) => name,
            None => format!("Unknown ({})", report.player_id),
        };

        let now = Utc::now().timestamp() as u64;
        let elapsed_secs = now.saturating_sub(report.created_at);
        let time_ago = if elapsed_secs < 60 {
            "just now".to_string()
        } else if elapsed_secs < 3600 {
            format!("{} minute(s) ago", elapsed_secs / 60)
        } else if elapsed_secs < 86400 {
            format!("{} hour(s) ago", elapsed_secs / 3600)
        } else {
            format!("{} day(s) ago", elapsed_secs / 86400)
        };

        self.send_message(
            &username,
            render!(
                REPORT_DETAIL,
                id = &report.id,
                reporter = &reporter,
                player = &report.target_player_id,
                reason = &report.reason,
                created = &time_ago
            ),
        )
        .await
        .map_err(|err| error!("Failed to send bug view message: {}", err))
        .ok();

        Ok(Response::new(ViewReportResponse { success: true }))
    }
}
