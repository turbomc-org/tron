use crate::BridgeService;
use crate::bridge::{RemoveTeamMemberRequest, RemoveTeamMemberResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_remove_team_member(
        &self,
        _request: Request<RemoveTeamMemberRequest>,
    ) -> Result<Response<RemoveTeamMemberResponse>, Status> {
        unimplemented!()
    }
}
