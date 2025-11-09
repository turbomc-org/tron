use crate::BridgeService;
use crate::bridge::{ViewBugRequest, ViewBugResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_view_bug(
        &self,
        request: Request<ViewBugRequest>,
    ) -> Result<Response<ViewBugResponse>, Status> {
        todo!("Implement view bug")
    }
}
