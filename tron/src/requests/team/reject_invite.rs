use crate::{BridgeService, config::messages::SUCCESSFULLY_REJECTED_TEAM_INVITATION, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{RejectTeamInviteRequest, RejectTeamInviteResponse};

impl BridgeService {
    pub async fn handle_reject_team_invite(
        &self,
        request: Request<RejectTeamInviteRequest>,
    ) -> Result<Response<RejectTeamInviteResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.team;

        info!(
            "Reject team invite request from player {} received",
            username
        );

        let mut player = self.player(&username).await?;
        let team = self.team(&username, &target).await?;

        player
            .reject_team_request(team.id, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!(
                    "Failed to reject team request of player {}: {}",
                    username,
                    err.to_string()
                );

                Status::internal("Failed to reject team request")
            })?;

        self.send_message(
            &username,
            render!(SUCCESSFULLY_REJECTED_TEAM_INVITATION, name = team.name),
        )
        .await;

        info!(
            "Reject team invite request from player {} completed",
            username
        );

        Ok(Response::new(RejectTeamInviteResponse { success: true }))
    }
}
