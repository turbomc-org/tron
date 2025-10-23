use std::collections::HashSet;

use crate::BridgeService;
use crate::bridge::{SellItemRequest, SellItemResponse};
use crate::models::shop_item::ShopItem;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_sell_item(
        &self,
        request: Request<SellItemRequest>,
    ) -> Result<Response<SellItemResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let sell_items = inner_request.items;

        info!("Sell item request from player {} received", username);

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let mut sum: u64 = 0;

        for sell_item in sell_items {
            let set: HashSet<String> = sell_item.enchantments.into_iter().collect();
            let item = ShopItem::find_shop_item_by_id_and_enchantments(
                &self.cache.shop_items,
                &sell_item.item_type,
                &set,
            )
            .ok_or_else(|| {
                error!(
                    "Item: {} which player {} is trying to sell not found in database",
                    sell_item.item_type, username,
                );

                Status::not_found("Item not found")
            })?;

            let price = item
                .sell(
                    sell_item.quantity,
                    &mut player,
                    &self.databases.players,
                    &self.cache.active_players,
                )
                .await
                .map_err(|err| {
                    error!(
                        "Failed to sell item {}: {}",
                        sell_item.item_type,
                        err.to_string()
                    );

                    Status::internal(format!("Failed to sell item {}", sell_item.item_type))
                })?;

            sum += price;
        }

        info!("Sell item request from player {} completed", username);

        Ok(Response::new(SellItemResponse { price: sum }))
    }
}
