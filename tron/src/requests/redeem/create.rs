use crate::BridgeService;
use crate::models::redeem::{Redeem, Reward};
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{CreateRedeemCodeRequest, CreateRedeemCodeResponse};

impl BridgeService {
    pub async fn handle_create_redeem_code(
        &self,
        request: Request<CreateRedeemCodeRequest>,
    ) -> Result<Response<CreateRedeemCodeResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let code = inner_request.code;
        let expires_at = inner_request.expires_at;
        let reward = inner_request.reward;

        info!(
            "Create redeem code request from player {} received",
            username
        );

        let player = self.player(&username).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can create redeems."),
                )
                .await;
        }

        if self.state().indexes.redeem.contains_key(&code) {
            return self
                .status(
                    &username,
                    Status::already_exists("Redeem code already exists."),
                )
                .await;
        }

        let reward = match Reward::try_from(reward.unwrap()) {
            Ok(reward) => reward,
            Err(_) => {
                return self
                    .status(&username, Status::invalid_argument("Invalid rewards."))
                    .await;
            }
        };

        let redeem = Redeem::new(code, expires_at, player.id, reward);

        if let Err(e) = redeem
            .insert(&self.collections().redeems, &self.state())
            .await
        {
            return self
                .status(&username, Status::internal(e.to_string()))
                .await;
        }

        info!(
            "Create redeem code request from player {} completed",
            username
        );

        Ok(Response::new(CreateRedeemCodeResponse { success: true }))
    }
}
