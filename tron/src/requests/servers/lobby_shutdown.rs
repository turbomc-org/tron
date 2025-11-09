use tonic::{Request, Response, Status};
use tracing::{debug, error};

use crate::{
    BridgeService,
    bridge::{LobbyShutdownRequest, LobbyShutdownResponse},
};

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_lobby_shutdown(
        &self,
        request: Request<LobbyShutdownRequest>,
    ) -> Result<Response<LobbyShutdownResponse>, Status> {
        let inner_request = request.into_inner();
        let client_id = inner_request.client_id;

        debug!("Lobby shutdown requested by client {}", client_id);

        if !self.state().servers.lobbies.contains(&client_id) {
            error!("Client {} is not in a lobby", client_id);
            return Err(Status::not_found("Client is not in a lobby"));
        }

        self.state().servers.lobbies.remove(&client_id);

        debug!("Lobby shutdown request by client {} completed", client_id);

        Ok(Response::new(LobbyShutdownResponse { success: true }))
    }
}
