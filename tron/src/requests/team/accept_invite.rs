use crate::config::messages::{CONNECTION_ESTABLISHED, SQUAD_LINK_ESTABLISHED};
use crate::{BridgeService, render};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{debug, error};
use tron_protos::{AcceptTeamInviteRequest, AcceptTeamInviteResponse};

impl BridgeService {
    pub async fn handle_accept_team_invite(
        &self,
        request: Request<AcceptTeamInviteRequest>,
    ) -> Result<Response<AcceptTeamInviteResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.team;

        debug!(
            "Accept team invite request from player {} received",
            username
        );

        debug!("Fetching the player from cache");
        let mut player = self.player(&username).await?;
        debug!("Fetching the team id");
        let team_id = self.state().check_team_request(&player, &target).await?;
        let now = Utc::now().timestamp() as u64;

        debug!("Accepting the team request");
        player
            .accept_team_request(
                team_id,
                now,
                &self.collections().players,
                &self.collections().teams,
                &self.state(),
            )
            .await
            .map_err(|err| {
                error!(
                    "Failed to accept team invite request from player {}: {}",
                    username, err
                );
                Status::internal("Failed to accept team invite request")
            })?;

        self.send_message(&username, render!(SQUAD_LINK_ESTABLISHED, team = &target))
            .await;

        let team = self
            .state()
            .get_team(team_id)
            .ok_or_else(|| Status::not_found("Team not found"))?;

        let team_broadcast_message = render!(CONNECTION_ESTABLISHED, username = &username);

        for member in team
            .members
            .iter()
            .filter(|(member_id, _)| *member_id != &player.id)
        {
            let member_username = self
                .state()
                .get_player_username(&member.0)
                .ok_or_else(|| Status::not_found("Member not found"))?;

            self.send_message(&member_username, team_broadcast_message.clone())
                .await;
        }

        debug!(
            "Accept team invite request from player {} completed",
            username
        );

        Ok(Response::new(AcceptTeamInviteResponse { success: true }))
    }
}
