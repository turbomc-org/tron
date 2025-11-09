use crate::BridgeService;
use crate::bridge::{ViewBugRequest, ViewBugResponse};
use crate::config::messages::BUG_DETAIL;
use crate::models::player::Role;
use crate::render;
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_view_bug(
        &self,
        request: Request<ViewBugRequest>,
    ) -> Result<Response<ViewBugResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let bug_id = inner_request.bug_id;

        info!("View bug {} requested by {}", bug_id, username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can view bugs."),
                )
                .await;
        }

        let bug = match self.collections().bugs.find_by_id(bug_id).await {
            Ok(bug) => bug,
            Err(err) => {
                error!("Failed to fetch bug {}: {}", bug_id, err);
                return self
                    .status(
                        &username,
                        Status::internal("Failed to fetch bug from database."),
                    )
                    .await;
            }
        };

        let Some(bug) = bug else {
            return self
                .status(
                    &username,
                    Status::not_found("Failed to fetch bug from database."),
                )
                .await;
        };

        let reporter = match self.state().get_player_username(&bug.player_id) {
            Some(name) => name,
            None => format!("Unknown ({})", bug.player_id),
        };

        let now = Utc::now().timestamp() as u64;
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

        // Send rich formatted bug info
        self.send_message(
            &username,
            render!(
                BUG_DETAIL,
                id = &bug.id,
                reporter = &reporter,
                description = &bug.description,
                created = &time_ago
            ),
        )
        .await
        .map_err(|err| error!("Failed to send bug view message: {}", err))
        .ok();

        Ok(Response::new(ViewBugResponse { success: true }))
    }
}
