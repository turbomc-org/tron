use crate::BridgeService;
use crate::bridge::{CreateShopItemRequest, CreateShopItemResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_create_shop_item(
        &self,
        request: Request<CreateShopItemRequest>,
    ) -> Result<Response<CreateShopItemResponse>, Status> {
        todo!("Implement create shop item")
    }
}
