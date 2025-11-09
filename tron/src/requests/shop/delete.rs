use crate::BridgeService;
use crate::bridge::{DeleteShopItemRequest, DeleteShopItemResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_delete_shop_item(
        &self,
        request: Request<DeleteShopItemRequest>,
    ) -> Result<Response<DeleteShopItemResponse>, Status> {
        todo!("Implement delete shop item")
    }
}
