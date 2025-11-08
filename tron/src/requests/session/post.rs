use crate::BridgeService;
use crate::bridge::{PlayerPostLoginRequest, PlayerPostLoginResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_player_post_login(
        &self,
        request: Request<PlayerPostLoginRequest>,
    ) -> Result<Response<PlayerPostLoginResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        todo!("Implement player post login endpoint")
    }
}
