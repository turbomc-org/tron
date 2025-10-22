use crate::BridgeService;
use crate::bridge::{AcceptTeamInviteRequest, AcceptTeamInviteResponse};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_accept_team_invite(
        &self,
        request: Request<AcceptTeamInviteRequest>,
    ) -> Result<Response<AcceptTeamInviteResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.team;

        info!(
            "Accept team invite request from player {} received",
            username
        );

        let player = self.cache.get_player_with_handling(&username).await?;
        let team_id = self.cache.check_team_request(&player, &target).await?;
        let now = Utc::now().timestamp() as u64;

        player
            .accept_team_request(team_id, now, &self.databases.players, &self.databases.teams)
            .await
            .map_err(|err| {
                error!(
                    "Failed to accept team invite for player {} of team {}: {}",
                    username,
                    target,
                    err.to_string()
                );
                Status::internal(format!(
                    "Failed to accept team invite for player {} of team {}: {}",
                    username,
                    target,
                    err.to_string()
                ))
            })?;

        self.cache
            .accept_team_request(&player, team_id, now)
            .await
            .map_err(|err| {
                error!(
                    "Failed to update team for player {} in cache: {}",
                    username,
                    err.to_string()
                );

                Status::internal(format!(
                    "Failed to update cache please rejoin to solve this issue: {}",
                    err.to_string()
                ))
            })?;

        info!(
            "Accept team invite request from player {} completed",
            username
        );

        Ok(Response::new(AcceptTeamInviteResponse { success: true }))
    }
}
