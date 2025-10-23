use crate::BridgeService;
use crate::bridge::{GetTeamMembersRequest, GetTeamMembersResponse};
use crate::models::team::Team;
use futures::future::join_all;
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    pub async fn handle_get_team_members(
        &self,
        request: Request<GetTeamMembersRequest>,
    ) -> Result<Response<GetTeamMembersResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Get team members request for player {} received", username);

        let player = self.cache.get_player_with_handling(&username).await?;
        let team = Team::get_team(&player, &self.cache.teams).await?;

        let members: Result<Vec<String>, Status> = {
            let futures = team.members.iter().map(|member| async move {
                self.cache
                    .get_player_username(member.0.clone())
                    .await
                    .map_err(|e| Status::internal(format!("Failed to get player username: {}", e)))
            });
            let results = join_all(futures).await;
            results.into_iter().collect()
        };

        Ok(Response::new(GetTeamMembersResponse { members: members? }))
    }
}
