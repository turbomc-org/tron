use crate::bridge::{TransferBalanceRequest, TransferBalanceResponse};
use crate::config::messages::{INCOMING_TRANSFER, TRANSFERRED};
use crate::models::player::Player;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

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

        let mut player = self.state().get_player_with_handling(&username).await?;
        let mut target = self.state().get_player_with_handling(&receiver).await?;

        if player.coins < amount {
            error!("Player {} does not have enough coins", player.username);

            return Err(Status::invalid_argument(format!(
                "You don't have enough coins"
            )));
        }

        debug!(
            "Transferring coins from player {} to player {}",
            player.username, target.username
        );

        Player::transfer_coins(
            &mut player,
            &mut target,
            amount,
            &self.collections().players,
            &self.state(),
        )
        .await
        .map_err(|e| {
            error!(
                "Failed to transfer coins from player {} to player {} : {}",
                player.username,
                target.username,
                e.to_string()
            );

            Status::internal(format!(
                "Failed to transfer coins to player {}",
                target.username,
            ))
        })?;

        self.send_message(
            &username,
            render!(TRANSFERRED, amount = &amount, target = &receiver),
        )
        .await
        .map_err(|err| {
            error!("Failed to send player message: {}", err);
        })
        .unwrap();

        self.send_message(
            &receiver,
            render!(INCOMING_TRANSFER, amount = &amount, sender = &username),
        )
        .await
        .map_err(|err| {
            error!("Failed to send player message: {}", err);
        })
        .unwrap();

        info!(
            "Transfer Balance request for player {} to {} completed",
            username, receiver
        );

        Ok(Response::new(TransferBalanceResponse { success: true }))
    }
}
