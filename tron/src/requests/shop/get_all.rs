use crate::BridgeService;
use crate::bridge::{GetAllItemsRequest, GetAllItemsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_all_items(
        &self,
        request: Request<GetAllItemsRequest>,
    ) -> Result<Response<GetAllItemsResponse>, Status> {
        todo!("handle get all items")
    }
}
