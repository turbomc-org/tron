use crate::bridge::{CreateShopItemRequest, CreateShopItemResponse};
use crate::config::messages::ITEM_CREATED;
use crate::models::shop_item::{Rarity, ShopItem};
use crate::{render, BridgeService};
use std::collections::HashSet;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_create_shop_item(
        &self,
        request: Request<CreateShopItemRequest>,
    ) -> Result<Response<CreateShopItemResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let name = inner_request.name;
        let description = inner_request.description;
        let price = inner_request.price;
        let item_id = inner_request.item_id;

        info!("Create item request from player {} received", username);

        let new_item = ShopItem::new(
            item_id.clone(),
            name.clone(),
            description,
            "".to_string(),
            HashSet::new(),
            Rarity::Common,
            price,
            0,
        );

        new_item
            .insert(&self.collections().shop_items, &self.state())
            .await
            .map_err(|err| {
                error!(
                    "Failed to create shop item {} due to: {}",
                    name,
                    err.to_string()
                );
                Status::internal(format!("Failed to create shop item {}", name))
            })?;

        self.send_message(
            &username,
            render!(ITEM_CREATED, item_name = name.clone(), item_id = item_id),
        )
        .await
        .map_err(|err| {
            error!("Failed to send player message: {}", err);
        })
        .unwrap();

        info!("Create item request from player {} completed", username);

        Ok(Response::new(CreateShopItemResponse { success: true }))
    }
}
