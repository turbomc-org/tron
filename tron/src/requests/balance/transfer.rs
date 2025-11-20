use crate::config::messages::{INCOMING_TRANSFER, TRANSFERRED};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{TransferBalanceRequest, TransferBalanceResponse};

impl BridgeService {
    pub async fn handle_transfer_balance(
        &self,
        request: Request<TransferBalanceRequest>,
    ) -> Result<Response<TransferBalanceResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.sender;
        let receiver = inner_request.receiver;
        let amount = inner_request.amount;

        info!(
            "Transfer Balance request for player {} to {} received",
            username, receiver
        );

        if username == receiver {
            error!("Player {} tried to transfer coins to themselves", username);

            return Err(Status::invalid_argument(format!(
                "You cannot transfer coins to yourself"
            )));
        }

        let mut player = self.player(&username).await?;
        let mut target = self.player(&receiver).await?;

        if player.coins < amount {
            error!("Player {} does not have enough coins", player.username);

            return self
                .status(
                    &username,
                    Status::out_of_range("You don't have enough coins."),
                )
                .await;
        }

        if let Err(e) = player
            .transfer_coins(
                &mut target,
                amount,
                &self.collections().players,
                &self.state(),
            )
            .await
        {
            error!(
                "Failed to transfer coins from player {} to player {} : {}",
                player.username,
                target.username,
                e.to_string()
            );

            return self
                .status(
                    &username,
                    Status::internal(format!(
                        "Failed to transfer coins to player {}",
                        target.username
                    )),
                )
                .await;
        }

        self.send_message(
            &username,
            render!(TRANSFERRED, amount = &amount, target = &receiver),
        )
        .await;

        self.send_message(
            &receiver,
            render!(INCOMING_TRANSFER, amount = &amount, sender = &username),
        )
        .await;

        info!(
            "Transfer Balance request for player {} to {} completed",
            username, receiver
        );

        Ok(Response::new(TransferBalanceResponse { success: true }))
    }
}
