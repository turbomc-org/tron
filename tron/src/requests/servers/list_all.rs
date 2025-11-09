use crate::BridgeService;
use crate::bridge::{ListAllServersRequest, ListAllServersResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_all_servers(
        &self,
        request: Request<ListAllServersRequest>,
    ) -> Result<Response<ListAllServersResponse>, Status> {
        todo!("Implement all servers")
    }
}
