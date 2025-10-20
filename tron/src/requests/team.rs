use crate::BridgeService;
use crate::bridge::{CreateTeamRequest, CreateTeamResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_create_team(
        &self,
        request: Request<CreateTeamRequest>,
    ) -> Result<Response<CreateTeamResponse>, Status> {
        unimplemented!()
    }
}
