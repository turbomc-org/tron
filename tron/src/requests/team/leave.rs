use crate::BridgeService;
use crate::bridge::{LeaveTeamRequest, LeaveTeamResponse};
use tonic::{Request, Response, Status};
use tracing::{debug, error};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_leave_team(
        &self,
        request: Request<LeaveTeamRequest>,
    ) -> Result<Response<LeaveTeamResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Leave team request for player {} received", username);

        let mut player = self.state.get_player_with_handling(&username).await?;

        if player.team.is_none() {
            error!(
                "Player {} tried to leave a team but is not in any team",
                username
            );

            return Err(Status::not_found("You are not in any team"));
        }

        let mut team = self
            .state
            .get_team_with_handling(player.team.unwrap())
            .await?;

        team.remove_member(
            &mut player,
            &self.collections.players,
            &self.collections.teams,
            &self.state,
        )
        .await
        .map_err(|e| {
            error!("Failed to leave team for player {}: {}", username, e);
            Status::internal("Failed to leave team")
        })?;

        self.send_message_to_player(
            &username,
            format!(
                "<gradient:#C724B1:#7A00FF><bold>✅ SQUAD LINK SEVERED</bold></gradient>\n\
             <gray>You have disconnected from the <white><bold>{}</bold></white> squad.</gray>\n\
             <dark_gray>»</dark_gray> <gray>You are now operating independently.</gray>",
                team.name
            ),
        )
        .await;

        let team_broadcast_message = format!(
            "<gradient:#C724B1:#7A00FF><bold>⚡ USER DISCONNECTED</bold></gradient>\n\
             <gray><white><bold>{}</bold></white> has severed their link to the squad.</gray>\n\
             <dark_gray>»</dark_gray> <click:run_command:'/team info'><u><gradient:#B200FF:#6A00A3>View updated roster</gradient></u></click>",
            username
        );

        for member in team
            .members
            .iter()
            .filter(|(member_id, _)| *member_id != &player.id)
        {
            let member_username = self
                .state
                .get_player_username(&member.0)
                .ok_or_else(|| Status::not_found("Member not found"))?;

            self.send_message_to_player(&member_username, team_broadcast_message.clone())
                .await
        }

        debug!("Leave team request for player {} completed", username);

        Ok(Response::new(LeaveTeamResponse { success: true }))
    }
}
