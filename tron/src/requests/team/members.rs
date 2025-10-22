use crate::BridgeService;
use crate::bridge::{GetTeamMembersRequest, GetTeamMembersResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_team_members(
        &self,
        _request: Request<GetTeamMembersRequest>,
    ) -> Result<Response<GetTeamMembersResponse>, Status> {
        unimplemented!()
    }
}
