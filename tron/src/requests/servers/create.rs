use crate::BridgeService;
use crate::bridge::{CreateServerRequest, CreateServerResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_create_server(
        &self,
        request: Request<CreateServerRequest>,
    ) -> Result<Response<CreateServerResponse>, Status> {
        todo!("Implement create server")
    }
}
