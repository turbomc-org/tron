use crate::bridge::{GetBalanceRequest, GetBalanceResponse};
use crate::config::messages::BALANCE;
use crate::{BridgeService, render};
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

        self.send_message_to_player(&username, render!(BALANCE, balance = &player.coins))
            .await;

        info!(
            "Successfully responded to Get Balance request for player {}",
            username
        );

        Ok(Response::new(GetBalanceResponse {
            balance: player.coins,
        }))
    }
}
