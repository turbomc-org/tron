use tonic::{Request, Response, Status};
use tracing::{error, warn};

use crate::{
    BridgeService,
    bridge::{ProxyShutdownRequest, ProxyShutdownResponse},
};

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_proxy_shutdown(
        &self,
        request: Request<ProxyShutdownRequest>,
    ) -> Result<Response<ProxyShutdownResponse>, Status> {
        let inner_request = request.into_inner();
        let client_id = inner_request.client_id;

        warn!("Proxy shutdown request from client {} received", client_id);

        if !self.state().servers.proxies.contains(&client_id) {
            error!("Client {} is not in a lobby", client_id);
            return Err(Status::not_found("Client is not in a lobby"));
        }

        self.state().servers.proxies.remove(&client_id);

        warn!("Proxy shutdown request from client {} completed", client_id);

        Ok(Response::new(ProxyShutdownResponse { success: true }))
    }
}
