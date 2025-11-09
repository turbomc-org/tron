use crate::bridge::ViewItemRequest;
use crate::{BridgeService, bridge::ViewItemResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_view_item(
        &self,
        request: Request<ViewItemRequest>,
    ) -> Result<Response<ViewItemResponse>, Status> {
        todo!("Implement view item")
    }
}
