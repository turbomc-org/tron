use crate::bridge::{GetTeamMembersRequest, GetTeamMembersResponse};
use crate::config::messages::SQUAD_ROSTER;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::debug;

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_get_team_members(
        &self,
        request: Request<GetTeamMembersRequest>,
    ) -> Result<Response<GetTeamMembersResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username.clone();

        debug!("Get team members request for player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;
        let team_id = player
            .team
            .ok_or_else(|| Status::not_found("You are not in a team"))?;

        let team = self.state().get_team_with_handling(team_id).await?;
        let leader_id = team.leader;

        let mut members = Vec::new();
        for member_id in &team.members {
            if let Some(member_name) = self.state().get_player_username(&member_id.0) {
                members.push(member_name);
            }
        }

        let mut roster_lines = Vec::new();
        for member_name in &members {
            let is_online = self.state().active_players.contains_key(member_name);

            let is_leader = if let Some(member) = self
                .state()
                .get_player_with_handling(member_name)
                .await
                .ok()
            {
                member.id == leader_id
            } else {
                false
            };

            let status_dot = if is_online {
                "<green>●</green>"
            } else {
                "<dark_gray>●</dark_gray>"
            };
            let leader_icon = if is_leader {
                "<gradient:#B200FF:#6A00A3>★ </gradient>"
            } else {
                ""
            };
            let player_color = if is_online { "<white>" } else { "<gray>" };

            roster_lines.push(format!(
                "<dark_gray> - {} {}{}{}</dark_gray>",
                status_dot, leader_icon, player_color, member_name
            ));
        }

        let roster_text = roster_lines.join("\n");

        self.send_message(
            &username,
            render!(
                SQUAD_ROSTER,
                team_name = &team.name,
                member_count = &members.len(),
                roster_text = &roster_text
            ),
        )
        .await;

        debug!("Get team members request for player {} completed", username);

        Ok(Response::new(GetTeamMembersResponse { members }))
    }
}
