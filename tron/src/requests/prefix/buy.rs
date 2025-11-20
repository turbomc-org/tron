use crate::config::messages::{ALREADY_OWN_PREFIX, ASSET_ACQUIRED, INSUFFICIENT_CREDITS};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{BuyPrefixRequest, BuyPrefixResponse};

impl BridgeService {
    pub async fn handle_buy_prefix(
        &self,
        request: Request<BuyPrefixRequest>,
    ) -> Result<Response<BuyPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let prefix_name = inner_request.prefix;

        info!("Buy prefix request from player {} received", username);

        let mut player = self.player(&username).await?;
        let prefix = self.state().get_prefix_with_handling(&prefix_name).await?;

        if player.prefixes.contains(&prefix.id) {
            self.send_message(
                &username,
                render!(ALREADY_OWN_PREFIX, username = &player.username),
            )
            .await;

            error!("Player already owns this prefix");
            return Err(Status::already_exists("You already owns this prefix"));
        }

        if player.coins < prefix.price {
            self.send_message(
                &username,
                render!(
                    INSUFFICIENT_CREDITS,
                    credits = &(prefix.price - player.coins)
                ),
            )
            .await;

            error!("Player does not have enough coins");
            return Err(Status::failed_precondition("You do not have enough coins"));
        }

        prefix
            .buy(&mut player, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!("Failed to buy prefix: {}", err);
                Status::internal("Failed to buy prefix")
            })?;

        self.send_message(
            &username,
            render!(
                ASSET_ACQUIRED,
                color = &prefix.color,
                text = &prefix.text,
                price = &prefix.price,
                name = &prefix_name
            ),
        )
        .await;

        info!("Buy prefix request from player {} completed", username);

        Ok(Response::new(BuyPrefixResponse { success: true }))
    }
}
