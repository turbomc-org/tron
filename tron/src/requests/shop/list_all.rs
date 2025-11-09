use crate::BridgeService;
use crate::bridge::{ListAllItemsRequest, ListAllItemsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_all_items(
        &self,
        request: Request<ListAllItemsRequest>,
    ) -> Result<Response<ListAllItemsResponse>, Status> {
        todo!("list all items")
    }
}
