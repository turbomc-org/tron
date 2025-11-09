use crate::BridgeService;
use crate::bridge::{GetAllReportsRequest, GetAllReportsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_all_reports(
        &self,
        request: Request<GetAllReportsRequest>,
    ) -> Result<Response<GetAllReportsResponse>, Status> {
        todo!("Implement get all bugs")
    }
}
