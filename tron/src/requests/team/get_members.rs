use crate::BridgeService;
use tonic::{Request, Response, Status};
use tron_protos::{GetTeamMembersRequest, GetTeamMembersResponse};

impl BridgeService {
    pub async fn handle_get_team_members(
        &self,
        request: Request<GetTeamMembersRequest>,
    ) -> Result<Response<GetTeamMembersResponse>, Status> {
        let request = request.into_inner();
        let username = request.username;

        let player = self.player(&username).await?;

        if !player.team.is_none() {
            return self
                .status(&username, Status::not_found("You are not in any team."))
                .await;
        }

        let team = self.team_by_id(&username, &player.team.unwrap()).await?;

        let members: Vec<String> = team
            .members
            .iter()
            .map(|v| {
                self.state()
                    .get_player_username(v.0)
                    .unwrap_or("Unknown".to_string())
            })
            .collect();

        Ok(Response::new(GetTeamMembersResponse { members }))
    }
}
