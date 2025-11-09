use crate::BridgeService;
use crate::bridge::{DemotePermsRequest, DemotePermsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_demote_perms(
        &self,
        request: Request<DemotePermsRequest>,
    ) -> Result<Response<DemotePermsResponse>, Status> {
        todo!("Implement demote perms")
    }
}
