use crate::config::messages::ITEM_SOLD;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{SellItemRequest, SellItemResponse};

impl BridgeService {
    pub async fn handle_sell_item(
        &self,
        request: Request<SellItemRequest>,
    ) -> Result<Response<SellItemResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let sell_items = inner_request.items;

        info!("Sell item request from player {} received", username);

        let mut player = self.player(&username).await?;
        let mut sum: u64 = 0;

        for sell_item in sell_items {
            let set: Vec<String> = sell_item.enchantments.into_iter().collect();
            let item = self
                .state()
                .get_by_typeid_and_enchantments(&sell_item.item_type, &set)
                .await?;

            let price = item
                .sell(
                    sell_item.quantity,
                    &mut player,
                    &self.collections().players,
                    &self.state(),
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

        self.send_message(&username, render!(ITEM_SOLD, price = sum))
            .await;

        info!("Sell item request from player {} completed", username);

        Ok(Response::new(SellItemResponse { price: sum }))
    }
}
