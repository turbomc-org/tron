use crate::BridgeService;
use crate::models::player::Role;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{Bug as CompiledBug, GetBugRequest, GetBugResponse};

impl BridgeService {
    pub async fn handle_get_bug(
        &self,
        request: Request<GetBugRequest>,
    ) -> Result<Response<GetBugResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let bug_id = inner_request.bug_id;

        info!(
            "Get bug request from player {} for bug {} received",
            username, bug_id
        );

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use get bug command."),
                )
                .await;
        }

        let bug = match self.collections().bugs.find_by_id(bug_id).await {
            Ok(bug) => bug,
            Err(_) => {
                return self
                    .status(
                        &username,
                        Status::internal("Failed to find bug in database."),
                    )
                    .await;
            }
        };

        if bug.is_none() {
            return self
                .status(&username, Status::not_found("Bug not found"))
                .await;
        }

        let bug = bug.unwrap();

        let Some(username) = self.state().get_player_username(&bug.player_id) else {
            return self
                .status(
                    &username,
                    Status::internal("Failed to find player username"),
                )
                .await;
        };

        info!(
            "Get bug request from player {} for bug {} completed",
            username, bug_id
        );

        Ok(Response::new(GetBugResponse {
            bug: Some(CompiledBug {
                id: bug.id,
                username: username,
                description: bug.description,
            }),
        }))
    }
}
