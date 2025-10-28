use crate::BridgeService;
use crate::bridge::{RemoveTeamMemberRequest, RemoveTeamMemberResponse};
use crate::models::team::Team;
use tonic::{Request, Response, Status};
use tracing::{error, info};

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

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let mut team = Team::get_team(&player, &self.cache.teams).await?;

        if team.leader != player.id {
            return Err(Status::permission_denied(
                "Only team leaders can remove members",
            ));
        }

        team.remove_member(
            &mut player,
            &self.collections.players,
            &self.collections.teams,
            &self.cache.active_players,
            &self.cache.teams,
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

        Ok(Response::new(RemoveTeamMemberResponse { success: true }))
    }
}
