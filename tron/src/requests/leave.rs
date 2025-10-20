use crate::BridgeService;
use crate::bridge::{PlayerLeaveRequest, PlayerLeaveResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_player_leave(
        &self,
        request: Request<PlayerLeaveRequest>,
    ) -> Result<Response<PlayerLeaveResponse>, Status> {
        unimplemented!()
    }
}
