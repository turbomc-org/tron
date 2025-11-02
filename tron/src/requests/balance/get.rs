use crate::BridgeService;
use crate::bridge::{GetBalanceRequest, GetBalanceResponse};
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Get Balance request for player {} received", username);

        let player = self.state.get_player_with_handling(&username).await?;

        self.send_message_to_player(
          &username,
          format!(
            "<gradient:#C724B1:#7A00FF><bold>¤ CREDIT BALANCE</bold></gradient>\n\
             <gray>You currently have <white><bold>{}</bold></white> Hash-Coins.</gray>\n\
             <dark_gray>»</dark_gray> <click:run_command:'/shop'><u><gradient:#B200FF:#6A00A3>Access the Network Market</gradient></u></click>",
             &player.coins
          ),
        ).await;

        info!(
            "Successfully responded to Get Balance request for player {}",
            username
        );

        Ok(Response::new(GetBalanceResponse {
            balance: player.coins,
        }))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::logger::Logger;
//     use crate::models::player::{Edition, Player};
//     use mongodb::bson::doc;

//     #[tokio::test]
//     async fn test_get_balance_from_cache() {
//         Logger::init(true).await;
//         let service = BridgeService::new().await;
//         let username = "ladiesman217".to_string();

//         let player = Player {
//             username: username.clone(),
//             coins: 500,
//             ..Player::new(username.clone(), Edition::Java)
//         };

//         service.cache.insert_player(player.clone()).await.unwrap();

//         let req = tonic::Request::new(crate::bridge::GetBalanceRequest {
//             username: username.clone(),
//         });

//         let resp = service.handle_get_balance(req).await.unwrap().into_inner();

//         service
//             .databases
//             .players
//             .delete_one(doc! {"username": username})
//             .await
//             .unwrap();
//         assert_eq!(resp.balance, 500);
//     }
// }
