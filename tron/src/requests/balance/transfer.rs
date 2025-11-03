use crate::bridge::{TransferBalanceRequest, TransferBalanceResponse};
use crate::config::messages::{INCOMING_TRANSFER, TRANSFERRED};
use crate::models::player::Player;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
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

        let mut player = self.state.get_player_with_handling(&username).await?;
        let mut target = self.state.get_player_with_handling(&receiver).await?;

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
            &self.collections.players,
            &self.state,
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

        self.send_message_to_player(
            &username,
            render!(TRANSFERRED, amount = &amount, target = &receiver),
        )
        .await;

        self.send_message_to_player(
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::logger::Logger;
//     use crate::{bridge::bridge_server::Bridge, models::player::Edition};
//     use mongodb::bson::doc;

//     #[tokio::test]
//     async fn test_handle_transfer_balance() {
//         Logger::init(true).await;
//         let sender_username = "vaibhav_7872".to_string();
//         let receiver_username = "vaibhavi_7872".to_string();
//         let service = BridgeService::new().await;

//         let sender_join_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
//             username: sender_username.clone(),
//             edition: Edition::Java as i32,
//         });

//         let sender_join_resp = service
//             .player_join(sender_join_req)
//             .await
//             .unwrap()
//             .into_inner();

//         assert!(sender_join_resp.success);

//         let receiver_join_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
//             username: receiver_username.clone(),
//             edition: Edition::Java as i32,
//         });

//         let receiver_join_resp = service
//             .player_join(receiver_join_req)
//             .await
//             .unwrap()
//             .into_inner();

//         assert!(receiver_join_resp.success);

//         service
//             .databases
//             .players
//             .update_one(
//                 doc! {"username": sender_username.clone()},
//                 doc! {"$set": {"coins": 500}},
//             )
//             .await
//             .unwrap();

//         let mut sender = service
//             .cache
//             .get_player(&sender_username)
//             .await
//             .unwrap()
//             .unwrap();
//         sender.coins = 500;
//         service.cache.insert_player(sender).await.unwrap();

//         let req = tonic::Request::new(crate::bridge::TransferBalanceRequest {
//             sender: sender_username.clone(),
//             receiver: receiver_username.clone(),
//             amount: 100,
//         });

//         let resp = service
//             .handle_transfer_balance(req)
//             .await
//             .unwrap()
//             .into_inner();

//         let sender_t = service
//             .cache
//             .get_player(&sender_username)
//             .await
//             .unwrap()
//             .unwrap();

//         assert_eq!(sender_t.coins, 400);

//         let receiver_db = service
//             .databases
//             .players
//             .find_one(doc! {"username": receiver_username.clone()})
//             .await
//             .unwrap()
//             .unwrap();

//         assert_eq!(receiver_db.coins, 100);

//         service
//             .databases
//             .players
//             .delete_one(doc! {"username": sender_username.clone()})
//             .await
//             .unwrap();

//         service
//             .databases
//             .players
//             .delete_one(doc! {"username": receiver_username.clone()})
//             .await
//             .unwrap();

//         assert!(resp.success);
//     }
// }
