use crate::BridgeService;
use crate::bridge::{GetTeamMembersRequest, GetTeamMembersResponse};
use futures::future::join_all;
use tonic::{Request, Response, Status};
use tracing::debug;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_get_team_members(
        &self,
        request: Request<GetTeamMembersRequest>,
    ) -> Result<Response<GetTeamMembersResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Get team members request for player {} received", username);

        let player = self.state.get_player_with_handling(&username).await?;

        if player.team.is_none() {
            return Err(Status::not_found("You are not in a team"));
        }

        let team = self
            .state
            .get_team_with_handling(player.team.unwrap())
            .await?;

        let futures = team.members.iter().map(|member| async move {
            self.state
                .get_player_username(&member.0.clone())
                .await
                .map_err(|e| Status::internal(format!("Failed to get player username: {}", e)))
        });

        let results = join_all(futures).await;

        let mut members = Vec::new();
        for res in results {
            match res {
                Ok(Some(username)) => members.push(username),
                Ok(None) => continue,
                Err(status) => return Err(status),
            }
        }

        debug!("Get team members request for player {} completed", username);

        Ok(Response::new(GetTeamMembersResponse { members: members }))
    }
}
