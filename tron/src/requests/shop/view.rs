use crate::BridgeService;
use crate::models::shop_item::ShopItem;
use tonic::{Request, Response, Status};
use tron_protos::{ViewItemRequest, ViewItemResponse};

impl BridgeService {
    pub async fn handle_view_item(
        &self,
        request: Request<ViewItemRequest>,
    ) -> Result<Response<ViewItemResponse>, Status> {
        let inner_request = request.into_inner();
        let id = inner_request.id;

        let shop_item = self.state().get_shop_item(&id).await?;

        let compiled_item = ShopItem::convert_shop_item(shop_item);

        Ok(Response::new(ViewItemResponse {
            item: Some(compiled_item),
        }))
    }
}
