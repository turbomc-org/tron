use crate::BridgeService;
use crate::bridge::{PlayerFloodgateLoginRequest, PlayerFloodgateLoginResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_player_floodgate_login(
        &self,
        request: Request<PlayerFloodgateLoginRequest>,
    ) -> Result<Response<PlayerFloodgateLoginResponse>, Status> {
        todo!("Implement player floodgate login endpoint")
    }
}
