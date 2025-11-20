pub mod create;
pub mod delete;
pub mod get_all;
pub mod list_all;
pub mod view;

use crate::config::messages::REDEEM;
use crate::utils::is_expired;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{RedeemCodeRequest, RedeemCodeResponse};

impl BridgeService {
    pub async fn handle_redeem_code(
        &self,
        request: Request<RedeemCodeRequest>,
    ) -> Result<Response<RedeemCodeResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let code = inner_request.code;

        info!("Redeem code request from player {} received", username);

        let mut player = self.player(&username).await?;

        if !self.state().indexes.redeem.contains_key(&code) {
            return self
                .status(&username, Status::not_found("Code you entered is invalid."))
                .await;
        }

        let id = match self.state().indexes.redeem.get(&code) {
            Some(id) => id.value().clone(),
            None => {
                return self
                    .status(&username, Status::internal("Failed to retrieve code ID"))
                    .await;
            }
        };

        if player.redeemed_codes.contains(&id) {
            return self
                .status(
                    &username,
                    Status::already_exists(
                        "Code you entered has already been redeemed once by you.",
                    ),
                )
                .await;
        }

        let redeem = match self.state().redeems.get(&id) {
            Some(r) => r.value().clone(),
            None => {
                return self
                    .status(
                        &username,
                        Status::internal("Failed to retrieve redeem code record."),
                    )
                    .await;
            }
        };

        if is_expired(redeem.expires_at) {
            return self
                .status(
                    &username,
                    Status::invalid_argument("Code you entered has expired."),
                )
                .await;
        }

        if let Err(e) = redeem
            .redeem(&mut player, &self.collections().players, &self.state())
            .await
        {
            error!("Failed to redeem code: {}", e);
            return self
                .status(&username, Status::internal("Failed to redeem code."))
                .await;
        }

        self.send_message(
            &username,
            render!(
                REDEEM,
                code = redeem.code,
                reward = redeem.reward.as_string()
            ),
        )
        .await;

        info!("Redeem code request from player {} completed", username);

        Ok(Response::new(RedeemCodeResponse { success: true }))
    }
}
