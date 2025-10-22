use crate::BridgeService;
use crate::bridge::{LeaveTeamRequest, LeaveTeamResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_leave_team(
        &self,
        _request: Request<LeaveTeamRequest>,
    ) -> Result<Response<LeaveTeamResponse>, Status> {
        unimplemented!()
    }
}
