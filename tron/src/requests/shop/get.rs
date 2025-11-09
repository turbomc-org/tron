use crate::BridgeService;
use crate::bridge::{GetItemRequest, GetItemResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_item(
        &self,
        request: Request<GetItemRequest>,
    ) -> Result<Response<GetItemResponse>, Status> {
        todo!("handle get item")
    }
}
