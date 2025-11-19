use crate::BridgeService;
use crate::models::shop_item::ShopItem;
use tonic::{Request, Response, Status};
use tron_protos::{GetAllItemsRequest, GetAllItemsResponse};

impl BridgeService {
    pub async fn handle_get_all_items(
        &self,
        request: Request<GetAllItemsRequest>,
    ) -> Result<Response<GetAllItemsResponse>, Status> {
        let _inner_request = request.into_inner(); // Request is empty, but still need to consume it

        let items: Vec<tron_protos::ShopItem> = self
            .state()
            .shop_items
            .iter()
            .map(|entry| ShopItem::convert_shop_item(entry.value().clone()))
            .collect();

        Ok(Response::new(GetAllItemsResponse { items }))
    }
}
