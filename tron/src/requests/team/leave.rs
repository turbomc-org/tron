use crate::BridgeService;
use crate::bridge::{LeaveTeamRequest, LeaveTeamResponse};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_leave_team(
        &self,
        request: Request<LeaveTeamRequest>,
    ) -> Result<Response<LeaveTeamResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Leave team request for player {} received", username);

        let mut player = self.cache.get_player_with_handling(&username).await?;

        if player.team.is_none() {
            error!(
                "Player {} tried to leave a team but is not in any team",
                username
            );

            return Err(Status::not_found("You are not in any team"));
        }

        let mut team = self.cache.get_team(player.team.unwrap()).await?;

        team.remove_member(
            &mut player,
            &self.collections.players,
            &self.collections.teams,
            &self.cache.active_players,
            &self.cache.teams,
        )
        .await
        .map_err(|e| {
            error!("Failed to leave team for player {}: {}", username, e);
            Status::internal("Failed to leave team")
        })?;

        info!("Leave team request for player {} completed", username);

        Ok(Response::new(LeaveTeamResponse { success: true }))
    }
}
