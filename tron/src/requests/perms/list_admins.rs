use crate::BridgeService;
use crate::bridge::{ListAllAdminsRequest, ListAllAdminsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_all_admins(
        &self,
        request: Request<ListAllAdminsRequest>,
    ) -> Result<Response<ListAllAdminsResponse>, Status> {
        todo!("Implement list all admins")
    }
}
