use crate::BridgeService;
use crate::bridge::{DeleteReportRequest, DeleteReportResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_delete_report(
        &self,
        request: Request<DeleteReportRequest>,
    ) -> Result<Response<DeleteReportResponse>, Status> {
        todo!("Implement delete report logic")
    }
}
