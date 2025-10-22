use crate::BridgeService;
use crate::bridge::{
    GetBalanceRequest, GetBalanceResponse, TransferBalanceRequest, TransferBalanceResponse,
};
use crate::models::player::Player;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use serde::{Deserialize, Serialize};
use tokio::sync::watch::error::RecvError;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    pub async fn handle_get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Get Balance request for player {} received", username);

        let player = self.cache.get_player_with_handling(&username).await?;

        info!(
            "Successfully responded to Get Balance request for player {}",
            username
        );

        Ok(Response::new(GetBalanceResponse {
            balance: player.coins,
        }))
    }

    pub async fn handle_transfer_balance(
        &self,
        request: Request<TransferBalanceRequest>,
    ) -> Result<Response<TransferBalanceResponse>, Status> {
        let players = &self.databases.players.clone();
        let inner_request = request.into_inner();
        let username = inner_request.sender;
        let mut player = self.cache.get_player_with_handling(&username).await?;
        let receiver = inner_request.receiver;
        let amount = inner_request.amount;

        debug!(
            "Transfer Balance request for player {} to {} received",
            username, receiver
        );

        #[derive(Serialize, Deserialize)]
        struct PartialResponse {
            #[serde(rename = "_id")]
            id: u64,
            coins: u64,
            username: String,
        }

        let partial_players: Collection<PartialResponse> = self.databases.players.clone_with_type();
        let projection = doc! { "_id": 1, "coins": 1, "username": 1 };
        let find_options = FindOneOptions::builder().projection(projection).build();

        debug!("Fetching receiver player {} from database", receiver);

        let receiver = partial_players
            .find_one(doc! {"username": &receiver})
            .with_options(find_options)
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch player {} from database : {}",
                    receiver,
                    e.to_string()
                );

                Status::internal(format!("Failed to fetch player {} from database", receiver))
            })?
            .ok_or_else(|| {
                error!("Player {} not found in database", receiver);

                Status::not_found(format!("Player {} not found in database", receiver))
            })?;

        if player.coins < amount {
            error!("Player {} does not have enough coins", player.username);

            return Err(Status::invalid_argument(format!(
                "Player {} does not have enough coins",
                player.username
            )));
        }

        debug!(
            "Transferring coins from player {} to player {}",
            player.username, receiver.username
        );

        Player::transfer_coins(player.id, receiver.id, amount, &players)
            .await
            .map_err(|e| {
                error!(
                    "Failed to transfer coins from player {} to player {} : {}",
                    player.username,
                    receiver.username,
                    e.to_string()
                );

                Status::internal(format!(
                    "Failed to transfer coins from player {} to player {}",
                    player.username, receiver.username
                ))
            })?;

        debug!("Current player coins: {}", player.coins);
        debug!("The amount: {}", amount);
        player.coins -= amount;
        debug!("Current player coins after transfer: {}", player.coins);
        self.cache
            .insert_player(player.clone())
            .await
            .map_err(|err| {
                error!(
                    "Failed to update coins of player {} in cache: {}",
                    player.username, err
                );
                Status::internal(format!(
                    "Failed to update coins of player {} in cache",
                    player.username
                ))
            })?;

        info!(
            "Successfully transferred coins from player {} to player {}",
            player.username, receiver.username
        );

        Ok(Response::new(TransferBalanceResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logger::Logger;
    use crate::{
        bridge::bridge_server::Bridge,
        models::player::{Edition, Player},
    };

    #[tokio::test]
    async fn test_get_balance_from_cache() {
        Logger::init(true).await;
        let service = BridgeService::new().await;
        let username = "ladiesman217".to_string();

        let player = Player {
            username: username.clone(),
            coins: 500,
            ..Player::new(username.clone(), Edition::Java)
        };

        service.cache.insert_player(player.clone()).await.unwrap();

        let req = tonic::Request::new(crate::bridge::GetBalanceRequest {
            username: username.clone(),
        });

        let resp = service.handle_get_balance(req).await.unwrap().into_inner();

        service
            .databases
            .players
            .delete_one(doc! {"username": username})
            .await
            .unwrap();
        assert_eq!(resp.balance, 500);
    }

    #[tokio::test]
    async fn test_handle_transfer_balance() {
        Logger::init(true).await;
        let sender_username = "vaibhav_7872".to_string();
        let receiver_username = "vaibhavi_7872".to_string();
        let service = BridgeService::new().await;

        let sender_join_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
            username: sender_username.clone(),
            edition: Edition::Java as i32,
        });

        let sender_join_resp = service
            .player_join(sender_join_req)
            .await
            .unwrap()
            .into_inner();

        assert!(sender_join_resp.success);

        let receiver_join_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
            username: receiver_username.clone(),
            edition: Edition::Java as i32,
        });

        let receiver_join_resp = service
            .player_join(receiver_join_req)
            .await
            .unwrap()
            .into_inner();

        assert!(receiver_join_resp.success);

        service
            .databases
            .players
            .update_one(
                doc! {"username": sender_username.clone()},
                doc! {"$set": {"coins": 500}},
            )
            .await
            .unwrap();

        let mut sender = service
            .cache
            .get_player(&sender_username)
            .await
            .unwrap()
            .unwrap();
        sender.coins = 500;
        service.cache.insert_player(sender).await.unwrap();

        let req = tonic::Request::new(crate::bridge::TransferBalanceRequest {
            sender: sender_username.clone(),
            receiver: receiver_username.clone(),
            amount: 100,
        });

        let resp = service
            .handle_transfer_balance(req)
            .await
            .unwrap()
            .into_inner();

        let sender_t = service
            .cache
            .get_player(&sender_username)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(sender_t.coins, 400);

        let receiver_db = service
            .databases
            .players
            .find_one(doc! {"username": receiver_username.clone()})
            .await
            .unwrap()
            .unwrap();

        assert_eq!(receiver_db.coins, 100);

        service
            .databases
            .players
            .delete_one(doc! {"username": sender_username.clone()})
            .await
            .unwrap();

        service
            .databases
            .players
            .delete_one(doc! {"username": receiver_username.clone()})
            .await
            .unwrap();

        assert!(resp.success);
    }
}
