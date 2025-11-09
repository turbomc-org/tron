use crate::BridgeService;
use crate::bridge::{GetBugRequest, GetBugResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_bug(
        &self,
        request: Request<GetBugRequest>,
    ) -> Result<Response<GetBugResponse>, Status> {
        todo!("Implement get bug")
    }
}
