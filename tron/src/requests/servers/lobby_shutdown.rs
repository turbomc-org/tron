use crate::BridgeService;
use crate::bridge::{LobbyShutdownRequest, LobbyShutdownResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_lobby_shutdown(
        &self,
        request: Request<LobbyShutdownRequest>,
    ) -> Result<Response<LobbyShutdownResponse>, Status> {
        todo!("Implement lobby shutdown logic")
    }
}
