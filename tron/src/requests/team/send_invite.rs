use crate::{
    BridgeService,
    config::messages::{NEW_TEAM_REQUEST, TEAM_REQUEST_SENT},
    render,
};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info, warn};
use tron_protos::{SendTeamInviteRequest, SendTeamInviteResponse};

impl BridgeService {
    pub async fn handle_send_team_invite(
        &self,
        request: Request<SendTeamInviteRequest>,
    ) -> Result<Response<SendTeamInviteResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        info!("Send invite request from player {} received", username);

        let player = self.player(&username).await?;

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

        let mut target_player = self.player(&target).await?;

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

        self.send_message(
            &username,
            render!(NEW_TEAM_REQUEST, sender = &username, name = &team.name),
        )
        .await;

        self.send_message(
            &username,
            render!(TEAM_REQUEST_SENT, receiver = target_player.username),
        )
        .await;

        info!("Send invite request from player {} completed", username);

        Ok(Response::new(SendTeamInviteResponse { success: true }))
    }
}
