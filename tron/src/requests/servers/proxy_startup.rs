use crate::GENERATOR;
use tonic::{Request, Response, Status};
use tracing::{info, warn};

use crate::{
    BridgeService,
    bridge::{ProxyStartupRequest, ProxyStartupResponse},
};

impl BridgeService {
    #[tracing::instrument]
    pub async fn handle_proxy_startup(
        &self,
        _request: Request<ProxyStartupRequest>,
    ) -> Result<Response<ProxyStartupResponse>, Status> {
        warn!("Proxy startup request received");

        let client_id = GENERATOR.generate();
        self.state.servers.proxies.insert(client_id);

        info!("New proxy client registered {}", client_id);
        warn!("Proxy startup request completed");

        Ok(Response::new(ProxyStartupResponse { client_id }))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::BridgeService;
//     use crate::bridge::ProxyStartupRequest;
//     use crate::logger::Logger;
//     use tonic::Request;

//     #[tokio::test]
//     async fn test_handle_proxy_startup() {
//         Logger::init(true).await;
//         let service = BridgeService::new().await;

//         let req = Request::new(ProxyStartupRequest {});

//         let resp = service
//             .handle_proxy_startup(req)
//             .await
//             .unwrap()
//             .into_inner();

//         let client_id = resp.client_id;

//         assert!(service.cache.servers.proxies.contains_key(&client_id));
//     }
// }
