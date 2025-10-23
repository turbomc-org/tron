use crate::BridgeService;
use crate::bridge::{JoinTeamRequest, JoinTeamResponse};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_join_team(
        &self,
        request: Request<JoinTeamRequest>,
    ) -> Result<Response<JoinTeamResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let team_name = inner_request.team;

        info!("Join team request for player {} received", username);

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let mut team = self.cache.get_team_by_name(&team_name).await?;

        if !team.open {
            error!("Player {} tried to join a closed team", username);

            return Err(Status::failed_precondition(
                "Team is not open for everyone to join ask leader to send you an invitation",
            ));
        }

        let now = Utc::now().timestamp() as u64;

        team.add_member(
            &mut player,
            now,
            &self.databases.players,
            &self.databases.teams,
            &self.cache.active_players,
            &self.cache.teams,
        )
        .await
        .map_err(|err| {
            error!("Failed to join team: {}", err);
            Status::internal("Failed to join team")
        })?;

        info!("Join team request for player {} completed", username);

        Ok(Response::new(JoinTeamResponse { success: true }))
    }
}
