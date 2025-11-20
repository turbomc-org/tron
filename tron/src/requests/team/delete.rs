use tonic::{Request, Response, Status};

use crate::{BridgeService, config::messages::DELETE_TEAM, render};
use tron_protos::{DeleteTeamRequest, DeleteTeamResponse};

impl BridgeService {
    pub async fn handle_delete_team(
        &self,
        request: Request<DeleteTeamRequest>,
    ) -> Result<Response<DeleteTeamResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let team_name = inner_request.team_name;

        let mut player = self.player(&username).await?;
        let mut team = self.team(&username, &team_name).await?;

        if team.leader != player.id {
            return self
                .status(
                    &username,
                    Status::permission_denied("You are not the leader of this team"),
                )
                .await;
        }

        if let Err(e) = team
            .remove_member(
                &mut player,
                &self.collections().players,
                &self.collections().teams,
                &self.state(),
            )
            .await
        {
            return self
                .status(
                    &username,
                    Status::internal(format!("Failed to remove member: {}", e)),
                )
                .await;
        }

        team.delete(&self.collections().teams, &self.state())
            .await
            .map_err(|err| Status::internal(format!("Failed to delete team: {}", err)))?;

        self.send_message(&username, render!(DELETE_TEAM, username = username))
            .await;

        Ok(Response::new(DeleteTeamResponse { success: true }))
    }
}
