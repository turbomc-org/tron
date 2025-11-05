use crate::BridgeService;
use crate::bridge::{PlayerPreLoginRequest, PlayerPreLoginResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_player_pre_login(
        &self,
        request: Request<PlayerPreLoginRequest>,
    ) -> Result<Response<PlayerPreLoginResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let edition = inner_request.edition;

        todo!("Implement the pre login endpoint")
    }
}
