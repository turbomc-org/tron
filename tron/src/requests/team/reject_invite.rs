use crate::BridgeService;
use crate::bridge::{RejectTeamInviteRequest, RejectTeamInviteResponse};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    #[tracing::instrument]
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

        let mut player = self.state.get_player_with_handling(&username).await?;
        let team_id = self
            .state
            .get_team_id(target.clone())
            .await
            .map_err(|err| {
                error!("Failed to get team ID for team {}", err);
                Status::internal(format!("Failed to find team {}", target))
            })?
            .ok_or_else(|| {
                error!("Requested team {} not found", target);
                Status::not_found(format!(
                    "Team {} not found, possible reason could be deletion",
                    target
                ))
            })?;

        player
            .reject_team_request(team_id, &self.collections.players, &self.state)
            .await
            .map_err(|err| {
                error!(
                    "Failed to reject team request of player {}: {}",
                    username,
                    err.to_string()
                );

                Status::internal("Failed to reject team request")
            })?;

        info!(
            "Reject team invite request from player {} completed",
            username
        );

        Ok(Response::new(RejectTeamInviteResponse { success: true }))
    }
}
