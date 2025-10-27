use tonic::{Request, Response, Status};
use tracing::warn;

use crate::{
    BridgeService,
    bridge::{LobbyShutdownRequest, LobbyShutdownResponse},
    models::servers::Client,
};

impl BridgeService {
    pub async fn handle_lobby_shutdown(
        &self,
        request: Request<LobbyShutdownRequest>,
    ) -> Result<Response<LobbyShutdownResponse>, Status> {
        let inner_request = request.into_inner();
        let client_id = inner_request.client_id;

        warn!("Lobby shutdown requested by client {}", client_id);

        let client = Client::get(&self.cache.servers.lobbies, client_id).await?;

        if !client {
            return Err(Status::not_found("Lobby server is not active"));
        }

        self.cache.servers.lobbies.remove(&client_id);

        warn!("Lobby shutdown request by client {} completed", client_id);

        Ok(Response::new(LobbyShutdownResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use crate::BridgeService;
    use crate::bridge::{LobbyShutdownRequest, LobbyStartupRequest};
    use crate::logger::Logger;
    use tonic::Request;

    #[tokio::test]
    async fn test_handle_lobby_shutdown() {
        Logger::init(true).await;
        let service = BridgeService::new().await;

        let s_req = Request::new(LobbyStartupRequest {});

        let s_resp = service
            .handle_lobby_startup(s_req)
            .await
            .unwrap()
            .into_inner();

        let client_id = s_resp.client_id;

        assert!(service.cache.servers.lobbies.contains_key(&client_id));

        let sh_req = Request::new(LobbyShutdownRequest { client_id });

        let s_resp = service
            .handle_lobby_shutdown(sh_req)
            .await
            .unwrap()
            .into_inner();

        assert!(s_resp.success);
        assert!(!service.cache.servers.lobbies.contains_key(&client_id));
    }
}
