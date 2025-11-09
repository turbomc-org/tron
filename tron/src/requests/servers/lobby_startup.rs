use crate::GENERATOR;
use tonic::{Request, Response, Status};
use tracing::{info, warn};

use crate::{
    BridgeService,
    bridge::{LobbyStartupRequest, LobbyStartupResponse},
};

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_lobby_startup(
        &self,
        request: Request<LobbyStartupRequest>,
    ) -> Result<Response<LobbyStartupResponse>, Status> {
        warn!("Lobby startup request received");

        let client_id = GENERATOR.generate();
        self.state().servers.lobbies.insert(client_id);

        info!("New lobby client registered {}", client_id);
        warn!("Lobby startup request completed");

        Ok(Response::new(LobbyStartupResponse { client_id }))
    }
}
