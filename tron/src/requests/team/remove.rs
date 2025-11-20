use crate::{
    BridgeService,
    config::messages::{KICKED_FROM_TEAM, REMOVED_TEAM_MEMBER},
    render,
};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{RemoveTeamMemberRequest, RemoveTeamMemberResponse};

impl BridgeService {
    pub async fn handle_remove_team_member(
        &self,
        request: Request<RemoveTeamMemberRequest>,
    ) -> Result<Response<RemoveTeamMemberResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        info!(
            "Remove team member request from player {} received",
            username
        );

        let mut player = self.player(&username).await?;

        if player.team.is_none() {
            return Err(Status::not_found("Player is not in a team"));
        }

        let mut team = self
            .state()
            .get_team_with_handling(player.team.unwrap())
            .await?;

        if team.leader != player.id {
            return Err(Status::permission_denied(
                "Only team leaders can remove members",
            ));
        }

        team.remove_member(
            &mut player,
            &self.collections().players,
            &self.collections().teams,
            &self.state(),
        )
        .await
        .map_err(|err| {
            error!(
                "Failed to remove player {} from team: {}",
                target,
                err.to_string()
            );

            Status::internal("Failed to remove player from team")
        })?;

        self.send_message(
            &username,
            render!(REMOVED_TEAM_MEMBER, username = &target, name = team.name),
        )
        .await;

        self.send_message(&target, render!(KICKED_FROM_TEAM, username = &target))
            .await;

        info!(
            "Remove team member request from player {} completed",
            username
        );

        Ok(Response::new(RemoveTeamMemberResponse { success: true }))
    }
}
