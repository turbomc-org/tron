use crate::BridgeService;
use crate::config::messages::{MODERATOR_PERMS_GAINED, PROMOTED_MODERATOR};
use crate::render;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{PromoteModeratorPermsRequest, PromoteModeratorPermsResponse};

impl BridgeService {
    pub async fn handle_promote_moderator_perms(
        &self,
        request: Request<PromoteModeratorPermsRequest>,
    ) -> Result<Response<PromoteModeratorPermsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        info!(
            "Promote moderator request from player {} received",
            username
        );

        let player = self.player(&username).await?;
        let mut target = self.player(&target).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can promote players to moderators."),
                )
                .await;
        }

        if target.is_moderator() {
            return self
                .status(
                    &username,
                    Status::already_exists("Target is already a moderator."),
                )
                .await;
        }

        if let Err(e) = target
            .promote_moderator(&self.collections().players, &self.state())
            .await
        {
            return self
                .status(&username, Status::internal(e.to_string()))
                .await;
        }

        self.send_message(
            &username,
            render!(PROMOTED_MODERATOR, username = target.username),
        )
        .await;

        self.send_message(
            &target.username,
            render!(MODERATOR_PERMS_GAINED, username = username),
        )
        .await;

        info!(
            "Promote moderator request from player {} completed",
            username
        );

        Ok(Response::new(PromoteModeratorPermsResponse {
            success: true,
        }))
    }
}
