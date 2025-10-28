use crate::BridgeService;
use crate::bridge::{PromoteTeamMemberRequest, PromoteTeamMemberResponse};
use crate::models::team::Team;
use tonic::{Request, Response, Status};
use tracing::{error, info};

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

        let player = self.cache.get_player_with_handling(&username).await?;
        let mut team = Team::get_team(&player, &self.cache.teams).await?;

        if team.leader != player.id {
            return Err(Status::permission_denied(
                "Only the team leader can promote members",
            ));
        }

        team.promote_player(player.id, &self.collections.teams, &self.cache.teams)
            .await
            .map_err(|err| {
                error!(
                    "Failed to promote player {} to leader: {}",
                    &target,
                    err.to_string()
                );

                Status::internal(format!("Failed to promote player {} to leader", &target))
            })?;

        info!(
            "Promote team member request from player {} checked",
            username
        );

        Ok(Response::new(PromoteTeamMemberResponse { success: true }))
    }
}
