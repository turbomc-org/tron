use crate::BridgeService;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{DeleteRedeemCodeRequest, DeleteRedeemCodeResponse};

impl BridgeService {
    pub async fn handle_delete_redeem_code(
        &self,
        request: Request<DeleteRedeemCodeRequest>,
    ) -> Result<Response<DeleteRedeemCodeResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let code = inner_request.code;

        info!(
            "Delete redeem code request from player {} received",
            username
        );

        let player = self.state().get_player_with_handling(&username).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can delete redeem codes."),
                )
                .await;
        }

        if !self.state().indexes.redeem.contains_key(&code) {
            return self
                .status(&username, Status::not_found("Redeem code not found."))
                .await;
        }

        let id = match self.state().indexes.redeem.get(&code) {
            Some(id) => id.value().clone(),
            None => {
                return self
                    .status(
                        &username,
                        Status::internal("Redeem code with ID not found."),
                    )
                    .await;
            }
        };

        let redeem = match self.state().redeems.get(&id) {
            Some(r) => r.value().clone(),
            None => {
                return self
                    .status(
                        &username,
                        Status::internal("Redeem code record with ID not found."),
                    )
                    .await;
            }
        };

        if let Err(e) = redeem
            .delete(&self.collections().redeems, &self.state())
            .await
        {
            error!("Failed to delete redeem code: {}", e);
            return self
                .status(&username, Status::internal("Failed to delete redeem code."))
                .await;
        }

        info!(
            "Delete redeem code request from player {} completed",
            username
        );

        Ok(Response::new(DeleteRedeemCodeResponse { success: true }))
    }
}
