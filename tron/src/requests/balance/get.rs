use crate::config::messages::BALANCE;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{GetBalanceRequest, GetBalanceResponse};

impl BridgeService {
    pub async fn handle_get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Get Balance request for player {} received", username);

        let player = self.player(&username).await?;

        self.send_message(&username, render!(BALANCE, balance = &player.coins))
            .await;

        info!("Get Balance request for player {} completed", username);

        Ok(Response::new(GetBalanceResponse {
            balance: player.coins,
        }))
    }
}
