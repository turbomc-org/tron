use crate::BridgeService;
use crate::bridge::{BuyItemRequest, BuyItemResponse};
use crate::models::shop_item::ShopItem;
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

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let item = self.cache.get_shop_item(&item_id).await?;

        item.buy(
            &mut player,
            &self.collections.players,
            &self.cache.active_players,
        )
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

        Ok(Response::new(BuyItemResponse {
            item: Some(ShopItem::convert_shop_item(item)),
        }))
    }
}
