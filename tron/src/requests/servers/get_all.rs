use crate::BridgeService;
use crate::bridge::{GetAllServersRequest, GetAllServersResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_all_servers(
        &self,
        request: Request<GetAllServersRequest>,
    ) -> Result<Response<GetAllServersResponse>, Status> {
        todo!("Implement get all servers")
    }
}
