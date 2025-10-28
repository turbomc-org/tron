use crate::GENERATOR;
use tonic::{Request, Response, Status};
use tracing::{info, warn};

use crate::{
    BridgeService,
    bridge::{LobbyStartupRequest, LobbyStartupResponse},
};

impl BridgeService {
    pub async fn handle_lobby_startup(
        &self,
        _request: Request<LobbyStartupRequest>,
    ) -> Result<Response<LobbyStartupResponse>, Status> {
        warn!("Lobby startup request received");

        let client_id = GENERATOR.generate();
        self.cache.servers.lobbies.insert(client_id, true);

        info!("New lobby client registered {}", client_id);
        warn!("Lobby startup request completed");

        Ok(Response::new(LobbyStartupResponse { client_id }))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::BridgeService;
//     use crate::bridge::LobbyStartupRequest;
//     use crate::logger::Logger;
//     use tonic::Request;

//     #[tokio::test]
//     async fn test_handle_lobby_startup() {
//         Logger::init(true).await;
//         let service = BridgeService::new().await;

//         let req = Request::new(LobbyStartupRequest {});

//         let resp = service
//             .handle_lobby_startup(req)
//             .await
//             .unwrap()
//             .into_inner();

//         let client_id = resp.client_id;

//         assert!(service.cache.servers.lobbies.contains_key(&client_id));
//     }
// }
