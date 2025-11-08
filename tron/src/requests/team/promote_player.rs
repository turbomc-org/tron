use crate::BridgeService;
use crate::bridge::{PromoteTeamMemberRequest, PromoteTeamMemberResponse};
use tonic::{Request, Response, Status};
use tracing::{debug, error};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_promote_team_member(
        &self,
        request: Request<PromoteTeamMemberRequest>,
    ) -> Result<Response<PromoteTeamMemberResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        debug!(
            "Promote team member request from player {} received",
            username
        );

        let player = self.state().get_player_with_handling(&username).await?;

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

        debug!(
            "Promote team member request from player {} completed",
            username
        );

        Ok(Response::new(PromoteTeamMemberResponse { success: true }))
    }
}
