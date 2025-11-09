use crate::BridgeService;
use crate::bridge::{GetAllModeratorsRequest, GetAllModeratorsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_all_moderators(
        &self,
        request: Request<GetAllModeratorsRequest>,
    ) -> Result<Response<GetAllModeratorsResponse>, Status> {
        todo!("Implement get all moderators")
    }
}
