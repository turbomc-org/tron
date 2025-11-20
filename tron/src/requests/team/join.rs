use crate::config::messages::{CONNECTION_ESTABLISHED, SQUAD_LINK_ESTABLISHED};
use crate::{BridgeService, render};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{JoinTeamRequest, JoinTeamResponse};

impl BridgeService {
    pub async fn handle_join_team(
        &self,
        request: Request<JoinTeamRequest>,
    ) -> Result<Response<JoinTeamResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let team_name = inner_request.team;

        info!("Join team request for player {} received", username);

        let mut player = self.player(&username).await?;

        if player.team.is_some() {
            error!("Player {} is already in a team", username);

            return Err(Status::failed_precondition(
                "You is already in a team. You must leave your current team before joining another.",
            ));
        }

        let mut team = self
            .state()
            .get_team_by_name(team_name.clone())
            .ok_or_else(|| Status::not_found(format!("Team {} not found", team_name)))?;

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
            &self.collections().players,
            &self.collections().teams,
            &self.state(),
        )
        .await
        .map_err(|err| {
            error!("Failed to join team: {}", err);
            Status::internal("Failed to join team")
        })?;

        self.send_message(
            &username,
            render!(SQUAD_LINK_ESTABLISHED, team = &team_name),
        )
        .await;

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

        info!("Join team request for player {} completed", username);

        Ok(Response::new(JoinTeamResponse { success: true }))
    }
}
