use crate::BridgeService;
use crate::bridge::{DemotePermsRequest, DemotePermsResponse};
use crate::models::player::Role;
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_demote_perms(
        &self,
        request: Request<DemotePermsRequest>,
    ) -> Result<Response<DemotePermsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        let player = self.state().get_player_with_handling(&username).await?;
        let target = self.state().get_player_with_handling(&target).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admin can execute demote permission command."),
                )
                .await;
        }

        if target.is_member() {
            return self
                .status(
                    &username,
                    Status::failed_precondition("The target don't have any permission yet."),
                )
                .await;
        }

        if let Err(e) = self
            .collections()
            .players
            .set_role(target.id, Role::Member)
            .await
        {
            return self
                .status(
                    &username,
                    Status::internal(format!("Failed to demote player: {}", e)),
                )
                .await;
        }

        Ok(Response::new(DemotePermsResponse { success: true }))
    }
}
