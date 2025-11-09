use crate::BridgeService;
use crate::bridge::{DeleteServerRequest, DeleteServerResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_delete_server(
        &self,
        request: Request<DeleteServerRequest>,
    ) -> Result<Response<DeleteServerResponse>, Status> {
        todo!("Implement delete server")
    }
}
