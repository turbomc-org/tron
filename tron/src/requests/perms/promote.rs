use crate::BridgeService;
use crate::bridge::{PromotePermsRequest, PromotePermsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_promote_perms(
        &self,
        request: Request<PromotePermsRequest>,
    ) -> Result<Response<PromotePermsResponse>, Status> {
        todo!("Implement promote permissions")
    }
}
