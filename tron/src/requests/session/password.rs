use crate::BridgeService;
use crate::bridge::{PlayerPasswordLoginRequest, PlayerPasswordLoginResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_player_password_login(
        &self,
        request: Request<PlayerPasswordLoginRequest>,
    ) -> Result<Response<PlayerPasswordLoginResponse>, Status> {
        todo!("Implement player floodgate login endpoint")
    }
}
