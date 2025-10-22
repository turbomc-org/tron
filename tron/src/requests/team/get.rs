use crate::BridgeService;
use crate::bridge::{GetTeamRequest, GetTeamResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_team(
        &self,
        request: Request<GetTeamRequest>,
    ) -> Result<Response<GetTeamResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request;

        unimplemented!()
    }
}
