use crate::BridgeService;
use crate::bridge::{ListAllModeratorsRequest, ListAllModeratorsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_all_moderators(
        &self,
        request: Request<ListAllModeratorsRequest>,
    ) -> Result<Response<ListAllModeratorsResponse>, Status> {
        todo!("Implement list all moderators")
    }
}
