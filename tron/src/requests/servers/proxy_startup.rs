use crate::GENERATOR;
use tonic::{Request, Response, Status};
use tracing::{info, warn};

use crate::{
    BridgeService,
    bridge::{ProxyStartupRequest, ProxyStartupResponse},
};

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_proxy_startup(
        &self,
        request: Request<ProxyStartupRequest>,
    ) -> Result<Response<ProxyStartupResponse>, Status> {
        warn!("Proxy startup request received");

        let client_id = GENERATOR.generate();
        self.state().servers.proxies.insert(client_id);

        info!("New proxy client registered {}", client_id);
        warn!("Proxy startup request completed");

        Ok(Response::new(ProxyStartupResponse { client_id }))
    }
}
