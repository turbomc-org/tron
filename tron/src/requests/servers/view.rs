use crate::BridgeService;
use crate::bridge::{ViewServerRequest, ViewServerResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_view_server(
        &self,
        request: Request<ViewServerRequest>,
    ) -> Result<Response<ViewServerResponse>, Status> {
        todo!("Implement view server")
    }
}
