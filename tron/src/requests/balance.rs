use crate::BridgeService;
use crate::bridge::{
    GetBalanceRequest, GetBalanceResponse, TransferBalanceRequest, TransferBalanceResponse,
};
use crate::models::player::Player;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let inner_request = request.into_inner();
        let players_cache = &self.cache.active_players.clone();
        let username = inner_request.username;

        let response = if players_cache.contains_key(&username) {
            let player = self
                .cache
                .get_player(&username)
                .await
                .map_err(|e| {
                    error!(
                        "Failed to fetch player {} from cache: {}",
                        username,
                        e.to_string()
                    );

                    Status::internal(format!("Failed to fetch player {} from cache", username))
                })?
                .ok_or_else(|| {
                    error!("Player {} not found in active players cache", username);

                    Status::data_loss(format!(
                        "Player {} not found in active players cache",
                        username
                    ))
                })?;

            Response::new(GetBalanceResponse {
                balance: player.coins,
            })
        } else {
            Response::new(GetBalanceResponse { balance: 0 })
        };

        Ok(response)
    }

    pub async fn handle_transfer_balance(
        &self,
        request: Request<TransferBalanceRequest>,
    ) -> Result<Response<TransferBalanceResponse>, Status> {
        let players = &self.databases.players.clone();
        let inner_request = request.into_inner();
        let username = inner_request.sender;
        let receiver = inner_request.receiver;
        let amount = inner_request.amount;

        let projection = doc! { "_id": 1, "coins": 1 };
        let find_options = FindOneOptions::builder().projection(projection).build();

        let sender = players
            .find_one(doc! {"username": &username})
            .with_options(find_options.clone())
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch player {} from database : {}",
                    username,
                    e.to_string()
                );

                Status::internal(format!("Failed to fetch player {} from database", username))
            })?
            .ok_or_else(|| {
                error!("Player {} not found in database", username);

                Status::not_found(format!("Player {} not found in database", username))
            })?;

        let receiver = players
            .find_one(doc! {"username": &receiver})
            .with_options(find_options)
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch player {} from database : {}",
                    username,
                    e.to_string()
                );

                Status::internal(format!("Failed to fetch player {} from database", username))
            })?
            .ok_or_else(|| {
                error!("Player {} not found in database", username);

                Status::not_found(format!("Player {} not found in database", username))
            })?;

        if sender.coins < amount {
            error!("Player {} does not have enough coins", sender.username);

            return Err(Status::invalid_argument(format!(
                "Player {} does not have enough coins",
                sender.username
            )));
        }

        Player::transfer_coins(sender.id, receiver.id, amount, &players)
            .await
            .map_err(|e| {
                error!(
                    "Failed to transfer coins from player {} to player {} : {}",
                    sender.username,
                    receiver.username,
                    e.to_string()
                );

                Status::internal(format!(
                    "Failed to transfer coins from player {} to player {}",
                    sender.username, receiver.username
                ))
            })?;

        Ok(Response::new(TransferBalanceResponse { success: true }))
    }
}
