use crate::BridgeService;
use crate::bridge::{ListAllBugsRequest, ListAllBugsResponse};
use crate::config::messages::{BUG_LIST, NO_BUGS_FOUND};
use crate::models::player::Role;
use crate::render;
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_list_all_bugs(
        &self,
        request: Request<ListAllBugsRequest>,
    ) -> Result<Response<ListAllBugsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use get all bug command."),
                )
                .await;
        }

        let bugs = match self.collections().bugs.all().await {
            Ok(bugs) => bugs,
            Err(err) => {
                error!("Failed to fetch bugs: {}", err);
                return self
                    .status(
                        &username,
                        Status::internal("Failed to fetch bugs from database."),
                    )
                    .await;
            }
        };

        if bugs.is_empty() {
            self.send_message(&username, render!(NO_BUGS_FOUND, username = &username))
                .await
                .map_err(|err| error!("Failed to send message: {}", err))
                .ok();
        } else {
            let now = Utc::now().timestamp() as u64;
            let mut entries = Vec::new();

            for bug in &bugs {
                let reporter = match self.state().get_player_username(&bug.player_id) {
                    Some(name) => name,
                    None => format!("Unknown ({})", bug.player_id),
                };

                let elapsed_secs = now.saturating_sub(bug.created_at);
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
                     <dark_gray>   └─</dark_gray> [<green><click:run_command:'/admin bug view {}'>View</click></green>] \
                     [<red><click:run_command:'/admin bug delete {}'>Delete</click></red>]",
                    bug.id, bug.description, reporter, time_ago, bug.id, bug.id
                ));
            }

            let list_str = entries.join("\n\n");

            self.send_message(
                &username,
                render!(BUG_LIST, count = &bugs.len(), list = &list_str),
            )
            .await
            .map_err(|err| error!("Failed to send message: {}", err))
            .ok();
        }

        Ok(Response::new(ListAllBugsResponse { success: true }))
    }
}
