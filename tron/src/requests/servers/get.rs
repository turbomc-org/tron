use crate::BridgeService;
use crate::bridge::{GetServerRequest, GetServerResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_server(
        &self,
        request: Request<GetServerRequest>,
    ) -> Result<Response<GetServerResponse>, Status> {
        todo!("Implement get server")
    }
}
