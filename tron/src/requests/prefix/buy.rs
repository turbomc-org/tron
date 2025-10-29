use crate::BridgeService;
use crate::bridge::{BuyPrefixRequest, BuyPrefixResponse};
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_buy_prefix(
        &self,
        request: Request<BuyPrefixRequest>,
    ) -> Result<Response<BuyPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let prefix_name = inner_request.prefix;

        let mut player = self.state.get_player_with_handling(&username).await?;
        let prefix = self.state.get_prefix_with_handling(&prefix_name).await?;

        if player.prefixes.contains(&prefix.id) {
            error!("Player already owns this prefix");
            return Err(Status::already_exists("You already owns this prefix"));
        }

        if player.coins < prefix.price {
            error!("Player does not have enough coins");
            return Err(Status::failed_precondition("You do not have enough coins"));
        }

        prefix
            .buy(&mut player, &self.collections.players, &self.state)
            .await
            .map_err(|err| {
                error!("Failed to buy prefix: {}", err);
                Status::internal("Failed to buy prefix")
            });

        Ok(Response::new(BuyPrefixResponse { success: true }))
    }
}
