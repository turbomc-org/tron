use crate::BridgeService;
use crate::bridge::{DeleteBugRequest, DeleteBugResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_delete_bug(
        &self,
        request: Request<DeleteBugRequest>,
    ) -> Result<Response<DeleteBugResponse>, Status> {
        todo!("Implement delete bug")
    }
}
