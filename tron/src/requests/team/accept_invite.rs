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

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let team_id = self.cache.check_team_request(&player, &target).await?;
        let now = Utc::now().timestamp() as u64;

        player
            .accept_team_request(
                team_id,
                now,
                &self.databases.players,
                &self.databases.teams,
                &self.cache.active_players,
                &self.cache.teams,
            )
            .await
            .map_err(|err| {
                error!(
                    "Failed to accept team invite request from player {}: {}",
                    username, err
                );
                Status::internal("Failed to accept team invite request")
            })?;

        info!(
            "Accept team invite request from player {} completed",
            username
        );

        Ok(Response::new(AcceptTeamInviteResponse { success: true }))
    }
}
