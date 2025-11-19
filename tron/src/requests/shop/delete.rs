use crate::config::messages::ITEM_DELETED;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{DeleteShopItemRequest, DeleteShopItemResponse};

impl BridgeService {
    pub async fn handle_delete_shop_item(
        &self,
        request: Request<DeleteShopItemRequest>,
    ) -> Result<Response<DeleteShopItemResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let id = inner_request.id;

        info!("Delete item request from player {} received", username);

        let shop_item = self.state().get_shop_item(&id).await?;
        let item_name = shop_item.name.clone();

        shop_item
            .remove(&self.collections().shop_items, &self.state())
            .await
            .map_err(|err| {
                error!(
                    "Failed to delete shop item {} due to: {}",
                    id,
                    err.to_string()
                );
                Status::internal(format!("Failed to delete shop item {}", id))
            })?;

        self.send_message(&username, render!(ITEM_DELETED, item_name = item_name))
            .await
            .map_err(|err| {
                error!("Failed to send player message: {}", err);
            })
            .unwrap();

        info!("Delete item request from player {} completed", username);

        Ok(Response::new(DeleteShopItemResponse { success: true }))
    }
}
