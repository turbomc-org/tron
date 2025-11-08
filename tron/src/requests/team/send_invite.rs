use crate::BridgeService;
use crate::bridge::{SendTeamInviteRequest, SendTeamInviteResponse};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{debug, error, warn};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_send_team_invite(
        &self,
        request: Request<SendTeamInviteRequest>,
    ) -> Result<Response<SendTeamInviteResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        debug!("Send invite request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.team.is_none() {
            error!("Player {} is not in a team", username);
            return Err(Status::not_found("You are not in a team"));
        }

        let team = self
            .state()
            .get_team_with_handling(player.team.unwrap())
            .await?;

        if team.leader != player.id {
            return Err(Status::permission_denied(
                "You are not the leader of this team",
            ));
        }

        let mut target_player = self.state().get_player_with_handling(&target).await?;

        if target_player.team.is_some() {
            return Err(Status::already_exists("Target player is already in a team"));
        }

        let now = Utc::now().timestamp() as u64;

        target_player
            .add_team_invite(team.id, now, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                warn!("Failed to send team invite to {}: {}", target, err);
                Status::internal("Failed to send team invite")
            })?;

        debug!("Send invite request from player {} completed", username);

        Ok(Response::new(SendTeamInviteResponse { success: true }))
    }
}
