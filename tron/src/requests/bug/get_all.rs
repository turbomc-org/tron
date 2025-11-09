use crate::BridgeService;
use crate::bridge::{GetAllBugsRequest, GetAllBugsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_all_bugs(
        &self,
        request: Request<GetAllBugsRequest>,
    ) -> Result<Response<GetAllBugsResponse>, Status> {
        todo!("Implement get all bugs")
    }
}
