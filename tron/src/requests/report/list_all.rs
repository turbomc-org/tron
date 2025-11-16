use crate::BridgeService;
use crate::bridge::{ListAllReportsRequest, ListAllReportsResponse};
use crate::config::messages::{NO_REPORTS_FOUND, REPORT_LIST};
use crate::models::player::Role;
use crate::render;
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_list_all_reports(
        &self,
        request: Request<ListAllReportsRequest>,
    ) -> Result<Response<ListAllReportsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("List all report request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin && player.role != Role::Moderator {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use get all bug command."),
                )
                .await;
        }

        let reports = match self.collections().reports.all().await {
            Ok(reports) => reports,
            Err(err) => {
                error!("Failed to fetch reports: {}", err);
                return self
                    .status(
                        &username,
                        Status::internal("Failed to fetch reports from database."),
                    )
                    .await;
            }
        };

        if reports.is_empty() {
            self.send_message(&username, render!(NO_REPORTS_FOUND, username = &username))
                .await
                .map_err(|err| error!("Failed to send message: {}", err))
                .ok();
        } else {
            let now = Utc::now().timestamp() as u64;
            let mut entries = Vec::new();

            for report in &reports {
                let reporter = match self.state().get_player_username(&report.player_id) {
                    Some(name) => name,
                    None => format!("Unknown ({})", report.player_id),
                };

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

                entries.push(format!(
                    "<dark_gray>»</dark_gray> <yellow><bold>#{}</bold></yellow> \
                     <gray>-</gray> <light_purple>{}</light_purple> \
                     <gray>by</gray> <aqua>{}</aqua> <gray>({})</gray>\n\
                     <dark_gray>   └─</dark_gray> [<green><click:run_command:'/admin report view {}'>View</click></green>] \
                     [<red><click:run_command:'/admin report delete {}'>Delete</click></red>]",
                    report.id, report.reason, reporter, time_ago, report.id, report.id
                ));
            }

            let list_str = entries.join("\n\n");

            self.send_message(
                &username,
                render!(REPORT_LIST, count = &reports.len(), list = &list_str),
            )
            .await
            .map_err(|err| error!("Failed to send message: {}", err))
            .ok();
        }

        info!("List all report request from player {} completed", username);

        Ok(Response::new(ListAllReportsResponse { success: true }))
    }
}
