use crate::BridgeService;
use crate::bridge::{GetAllAdminsRequest, GetAllAdminsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_all_admins(
        &self,
        request: Request<GetAllAdminsRequest>,
    ) -> Result<Response<GetAllAdminsResponse>, Status> {
        todo!("Implement get all admins")
    }
}
