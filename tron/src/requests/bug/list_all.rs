use crate::BridgeService;
use crate::bridge::{ListAllBugsRequest, ListAllBugsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_all_bugs(
        &self,
        request: Request<ListAllBugsRequest>,
    ) -> Result<Response<ListAllBugsResponse>, Status> {
        todo!("Implement list all bugs")
    }
}
