use crate::BridgeService;
use crate::bridge::{ViewReportRequest, ViewReportResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_view_report(
        &self,
        request: Request<ViewReportRequest>,
    ) -> Result<Response<ViewReportResponse>, Status> {
        todo!("Implement handle view bug")
    }
}
