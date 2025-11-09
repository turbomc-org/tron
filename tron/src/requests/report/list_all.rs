use crate::BridgeService;
use crate::bridge::{ListAllReportsRequest, ListAllReportsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_all_reports(
        &self,
        request: Request<ListAllReportsRequest>,
    ) -> Result<Response<ListAllReportsResponse>, Status> {
        todo!("Implement list all reports")
    }
}
