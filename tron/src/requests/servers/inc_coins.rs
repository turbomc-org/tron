use crate::bridge::{IncreaseCoinsRequest, IncreaseCoinsResponse};
use crate::config::messages::{ADMINISTRATIVE_GRANT, MASTER_CONTROL_CREDITS_GRANTED};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_increase_coins(
        &self,
        request: Request<IncreaseCoinsRequest>,
    ) -> Result<Response<IncreaseCoinsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;
        let amount = inner_request.amount;

        let _ = self.state().get_player_with_handling(&username).await?;
        let mut target_player = self.state().get_player_with_handling(&target).await?;

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

        if let Err(e) = self
            .send_message(
                &username,
                render!(
                    MASTER_CONTROL_CREDITS_GRANTED,
                    amount = &amount,
                    target = &target
                ),
            )
            .await
        {
            error!("Failed to send player {} message: {}", username, e);
        };

        if let Err(e) = self
            .send_message(&target, render!(ADMINISTRATIVE_GRANT, amount = &amount))
            .await
        {
            error!("Failed to send player {} message: {}", target, e);
        };

        Ok(Response::new(IncreaseCoinsResponse { success: true }))
    }
}
