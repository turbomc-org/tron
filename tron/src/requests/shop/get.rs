use crate::BridgeService;
use crate::models::shop_item::ShopItem;
use tonic::{Request, Response, Status};
use tron_protos::{GetItemRequest, GetItemResponse};

impl BridgeService {
    pub async fn handle_get_item(
        &self,
        request: Request<GetItemRequest>,
    ) -> Result<Response<GetItemResponse>, Status> {
        let inner_request = request.into_inner();
        let id = inner_request.id;

        let shop_item = self.state().get_shop_item(&id).await?;

        let compiled_item = ShopItem::convert_shop_item(shop_item);

        Ok(Response::new(GetItemResponse {
            item: Some(compiled_item),
        }))
    }
}
