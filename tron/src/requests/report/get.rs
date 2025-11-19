use crate::BridgeService;
use tron_protos::{GetReportRequest, GetReportResponse, Report as CompiledReport};
use crate::models::player::Role;
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_report(
        &self,
        request: Request<GetReportRequest>,
    ) -> Result<Response<GetReportResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let report_id = inner_request.id;

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use get bug command."),
                )
                .await;
        }

        let report = match self.collections().reports.find_by_id(report_id).await {
            Ok(report) => report,
            Err(_) => {
                return self
                    .status(
                        &username,
                        Status::internal("Failed to find report in database."),
                    )
                    .await;
            }
        };

        if report.is_none() {
            return self
                .status(&username, Status::not_found("Bug not found"))
                .await;
        }

        let report = report.unwrap();

        let Some(username) = self.state().get_player_username(&report.player_id) else {
            return self
                .status(
                    &username,
                    Status::internal("Failed to find player username"),
                )
                .await;
        };

        let Some(target_username) = self.state().get_player_username(&report.target_player_id)
        else {
            return self
                .status(
                    &username,
                    Status::internal("Failed to find target player username"),
                )
                .await;
        };

        Ok(Response::new(GetReportResponse {
            report: Some(CompiledReport {
                id: report.id,
                username,
                target_username,
                reason: report.reason,
            }),
        }))
    }
}
