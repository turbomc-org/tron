use crate::bridge::{BuyItemRequest, BuyItemResponse};
use crate::config::messages::ITEM_PURCHASED;
use crate::models::shop_item::ShopItem;
use crate::{render, BridgeService};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_buy_item(
        &self,
        request: Request<BuyItemRequest>,
    ) -> Result<Response<BuyItemResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let item_id = inner_request.item;

        info!("Buy item request from player {} received", username);

        let mut player = self.state().get_player_with_handling(&username).await?;
        let item = self.state().get_shop_item(&item_id).await?;

        let item_name = item.name.clone();
        let price = item.buy_price;

        item.buy(&mut player, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!(
                    "Failed to buy item {} requested by {} due to: {}",
                    item_id,
                    username,
                    err.to_string()
                );

                Status::internal(format!("Failed to buy item {}", item_id))
            })?;

        self.send_message(
            &username,
            render!(ITEM_PURCHASED, item_name = item_name, price = price),
        )
        .await
        .map_err(|err| {
            error!("Failed to send player message: {}", err);
        })
        .unwrap();

        info!("Buy item request from player {} completed", username);

        Ok(Response::new(BuyItemResponse {
            item: Some(ShopItem::convert_shop_item(item)),
        }))
    }
}
