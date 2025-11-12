use crate::BridgeService;
use crate::bridge::{LobbyStartupRequest, LobbyStartupResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_lobby_startup(
        &self,
        request: Request<LobbyStartupRequest>,
    ) -> Result<Response<LobbyStartupResponse>, Status> {
        todo!("Implement lobby startup handling")
    }
}
