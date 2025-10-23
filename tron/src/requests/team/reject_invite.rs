use crate::BridgeService;
use crate::bridge::{RejectTeamInviteRequest, RejectTeamInviteResponse};
use tonic::{Request, Response, Status};
use tracing::{error, info};

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

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let team_id = self.cache.check_team_request(&player, &target).await?;

        player
            .reject_team_request(team_id, &self.databases.players, &self.cache.active_players)
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
