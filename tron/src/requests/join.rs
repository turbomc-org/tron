use crate::BridgeService;
use crate::bridge::{PlayerJoinRequest, PlayerJoinResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_player_join(
        &self,
        request: Request<PlayerJoinRequest>,
    ) -> Result<Response<PlayerJoinResponse>, Status> {
        unimplemented!()
    }
}
