use crate::BridgeService;
use crate::bridge::{PromoteAdminPermsRequest, PromoteAdminPermsResponse};
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_promote_admin_perms(
        &self,
        request: Request<PromoteAdminPermsRequest>,
    ) -> Result<Response<PromoteAdminPermsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let token = inner_request.token;

        let mut player = self.state().get_player_with_handling(&username).await?;

        if player.is_admin() {
            return self
                .status(
                    &username,
                    Status::already_exists("You are already an admin"),
                )
                .await?;
        }

        if token == 58979323846 {
            if let Err(e) = player
                .promote_admin(&self.collections().players, &self.state())
                .await
            {
                error!("Failed to promote to admin: {}", e);
                return self
                    .status(&username, Status::internal("Failed to save player"))
                    .await;
            }
        } else {
            return self
                .status(&username, Status::unauthenticated("Invalid token."))
                .await;
        }

        Ok(Response::new(PromoteAdminPermsResponse { success: true }))
    }
}
