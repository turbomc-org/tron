use crate::config::messages::PROMOTE_TO_LEADER;
use crate::render;
use crate::{BridgeService, config::messages::GAINED_LEADER};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{PromoteTeamMemberRequest, PromoteTeamMemberResponse};

impl BridgeService {
    pub async fn handle_promote_team_member(
        &self,
        request: Request<PromoteTeamMemberRequest>,
    ) -> Result<Response<PromoteTeamMemberResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        info!(
            "Promote team member request from player {} received",
            username
        );

        let player = self.player(&username).await?;

        if player.team.is_none() {
            return Err(Status::not_found("You are not in a team"));
        }

        let mut team = self
            .state()
            .get_team_with_handling(player.team.unwrap())
            .await?;

        if team.leader != player.id {
            return Err(Status::permission_denied(
                "Only the team leader can promote members",
            ));
        }

        team.promote_player(player.id, &self.collections().teams, &self.state())
            .await
            .map_err(|err| {
                error!(
                    "Failed to promote player {} to leader: {}",
                    &target,
                    err.to_string()
                );

                Status::internal(format!("Failed to promote player {} to leader", &target))
            })?;

        self.send_message(&username, render!(PROMOTE_TO_LEADER, username = &target))
            .await;

        self.send_message(&target, render!(GAINED_LEADER, name = team.name))
            .await;

        info!(
            "Promote team member request from player {} completed",
            username
        );

        Ok(Response::new(PromoteTeamMemberResponse { success: true }))
    }
}
