use crate::BridgeService;
use crate::bridge::{GetReportRequest, GetReportResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_report(
        &self,
        request: Request<GetReportRequest>,
    ) -> Result<Response<GetReportResponse>, Status> {
        todo!("Implement get report logic")
    }
}
