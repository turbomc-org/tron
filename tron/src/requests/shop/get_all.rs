use crate::BridgeService;
use crate::bridge::{GetAllItemsRequest, GetAllItemsResponse};
use crate::models::shop_item::ShopItem;
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_all_items(
        &self,
        request: Request<GetAllItemsRequest>,
    ) -> Result<Response<GetAllItemsResponse>, Status> {
        let _inner_request = request.into_inner(); // Request is empty, but still need to consume it

        let items: Vec<crate::bridge::ShopItem> = self
            .state()
            .shop_items
            .iter()
            .map(|entry| ShopItem::convert_shop_item(entry.value().clone()))
            .collect();

        Ok(Response::new(GetAllItemsResponse { items }))
    }
}
