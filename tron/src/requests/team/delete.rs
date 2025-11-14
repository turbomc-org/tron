use tonic::{Request, Response, Status};

use crate::BridgeService;
use crate::bridge::{DeleteTeamRequest, DeleteTeamResponse};

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_delete_team(
        &self,
        request: Request<DeleteTeamRequest>,
    ) -> Result<Response<DeleteTeamResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let team_name = inner_request.team_name;

        let _ = self.state().get_player_with_handling(&username).await?;
        let team = self.state().get_team_by_name(team_name);

        if team.is_none() {
            return Err(Status::not_found(
                "Team requested to delete not found in database",
            ));
        }

        team.unwrap()
            .delete(&self.collections().teams, &self.state())
            .await
            .map_err(|err| Status::internal(format!("Failed to delete team: {}", err)))?;

        Ok(Response::new(DeleteTeamResponse { success: true }))
    }
}
