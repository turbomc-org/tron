use crate::BridgeService;
use crate::bridge::{JoinTeamRequest, JoinTeamResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_join_team(
        &self,
        _request: Request<JoinTeamRequest>,
    ) -> Result<Response<JoinTeamResponse>, Status> {
        unimplemented!()
    }
}
