use crate::BridgeService;
use crate::bridge::{SendTeamInviteRequest, SendTeamInviteResponse};
use crate::models::team::Team;
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{info, warn};

impl BridgeService {
    pub async fn handle_send_team_invite(
        &self,
        request: Request<SendTeamInviteRequest>,
    ) -> Result<Response<SendTeamInviteResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        info!("Send invite request from player {} received", username);

        let player = self.cache.get_player_with_handling(&username).await?;
        let team = Team::get_team(&player, &self.cache.teams).await?;

        if team.leader != player.id {
            return Err(Status::permission_denied(
                "You are not the leader of this team",
            ));
        }

        let mut target_player = self.cache.get_player_with_handling(&target).await?;

        if target_player.team.is_some() {
            return Err(Status::already_exists("Target player is already in a team"));
        }

        let now = Utc::now().timestamp() as u64;

        target_player
            .add_team_invite(
                team.id,
                now,
                &self.databases.players,
                &self.cache.active_players,
            )
            .await
            .map_err(|err| {
                warn!("Failed to send team invite to {}: {}", target, err);
                Status::internal("Failed to send team invite")
            })?;

        Ok(Response::new(SendTeamInviteResponse { success: true }))
    }
}
