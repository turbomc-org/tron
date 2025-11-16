use crate::bridge::{LeaveTeamRequest, LeaveTeamResponse};
use crate::config::messages::{SQUAD_LINK_SEVERED, USER_DISCONNECTED};
use crate::{BridgeService, render};
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

        let mut player = self.state().get_player_with_handling(&username).await?;

        if player.team.is_none() {
            error!(
                "Player {} tried to leave a team but is not in any team",
                username
            );

            return Err(Status::not_found("You are not in any team"));
        }

        let mut team = self
            .state()
            .get_team_with_handling(player.team.unwrap())
            .await?;

        team.remove_member(
            &mut player,
            &self.collections().players,
            &self.collections().teams,
            &self.state(),
        )
        .await
        .map_err(|e| {
            error!("Failed to leave team for player {}: {}", username, e);
            Status::internal("Failed to leave team")
        })?;

        if let Err(e) = self
            .send_message(&username, render!(SQUAD_LINK_SEVERED, team = &team.name))
            .await
        {
            error!("Failed to send message to player {}: {}", username, e);
        };

        let team_broadcast_message = render!(USER_DISCONNECTED, username = &username);

        for member in team
            .members
            .iter()
            .filter(|(member_id, _)| *member_id != &player.id)
        {
            let member_username = self
                .state()
                .get_player_username(&member.0)
                .ok_or_else(|| Status::not_found("Member not found"))?;

            if let Err(e) = self
                .send_message(&member_username, team_broadcast_message.clone())
                .await
            {
                error!(
                    "Failed to send message to player {}: {}",
                    member_username, e
                );
            };
        }

        info!("Leave team request for player {} completed", username);

        Ok(Response::new(LeaveTeamResponse { success: true }))
    }
}
