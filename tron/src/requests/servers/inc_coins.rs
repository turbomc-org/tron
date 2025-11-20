use crate::config::messages::{ADMINISTRATIVE_GRANT, MASTER_CONTROL_CREDITS_GRANTED};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{IncreaseCoinsRequest, IncreaseCoinsResponse};

impl BridgeService {
    pub async fn handle_increase_coins(
        &self,
        request: Request<IncreaseCoinsRequest>,
    ) -> Result<Response<IncreaseCoinsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;
        let amount = inner_request.amount;

        info!("Inc coins request from player {} received", username);

        let player = self.player(&username).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can increase coins."),
                )
                .await;
        }

        let mut target_player = self.player(&target).await?;

        target_player
            .inc_coins(amount, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!(
                    "Failed to increase {} coins to {}'s balance: {}",
                    amount, target, err
                );

                Status::internal(format!(
                    "Failed to increase {} coins to {}'s balance",
                    amount, target,
                ))
            })?;

        self.send_message(
            &username,
            render!(
                MASTER_CONTROL_CREDITS_GRANTED,
                amount = &amount,
                target = &target
            ),
        )
        .await;

        self.send_message(&target, render!(ADMINISTRATIVE_GRANT, amount = &amount))
            .await;

        info!("Inc coins request from player {} completed", username);

        Ok(Response::new(IncreaseCoinsResponse { success: true }))
    }
}
