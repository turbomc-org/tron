use crate::BridgeService;
use crate::bridge::{RejectTeamInviteRequest, RejectTeamInviteResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_reject_team_invite(
        &self,
        _request: Request<RejectTeamInviteRequest>,
    ) -> Result<Response<RejectTeamInviteResponse>, Status> {
        unimplemented!()
    }
}
