use crate::BridgeService;
use crate::bridge::{SendTeamInviteRequest, SendTeamInviteResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_send_team_invite(
        &self,
        _request: Request<SendTeamInviteRequest>,
    ) -> Result<Response<SendTeamInviteResponse>, Status> {
        unimplemented!()
    }
}
