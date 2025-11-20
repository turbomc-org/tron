use crate::config::messages::GAINED_MASTER_CONTROL;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{PromoteAdminPermsRequest, PromoteAdminPermsResponse};

impl BridgeService {
    pub async fn handle_promote_admin_perms(
        &self,
        request: Request<PromoteAdminPermsRequest>,
    ) -> Result<Response<PromoteAdminPermsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let token = inner_request.token;

        info!("Promote admin request from player {} received", username);

        let mut player = self.player(&username).await?;

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

            self.send_message(
                &username,
                render!(GAINED_MASTER_CONTROL, username = username),
            )
            .await;
        } else {
            return self
                .status(&username, Status::unauthenticated("Invalid token."))
                .await;
        }

        info!("Promote admin request from player {} completed", username);

        Ok(Response::new(PromoteAdminPermsResponse { success: true }))
    }
}
