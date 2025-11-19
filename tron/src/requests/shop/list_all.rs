use crate::BridgeService;
use tonic::{Request, Response, Status};
use tron_protos::{ListAllItemsRequest, ListAllItemsResponse};

impl BridgeService {
    pub async fn handle_list_all_items(
        &self,
        request: Request<ListAllItemsRequest>,
    ) -> Result<Response<ListAllItemsResponse>, Status> {
        let _inner_request = request.into_inner(); // Request is empty, but still need to consume it
        Ok(Response::new(ListAllItemsResponse { success: true }))
    }
}
