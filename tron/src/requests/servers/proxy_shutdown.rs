use crate::BridgeService;
use tonic::{Request, Response, Status};
use tracing::warn;
use tron_protos::{ProxyShutdownRequest, ProxyShutdownResponse};

impl BridgeService {
    pub async fn handle_proxy_shutdown(
        &self,
        request: Request<ProxyShutdownRequest>,
    ) -> Result<Response<ProxyShutdownResponse>, Status> {
        let inner_request = request.into_inner();
        let client_id = inner_request.client_id;

        if !self.state().proxies.contains(&client_id) {
            return Err(Status::not_found("Id not found"));
        }

        self.state().proxies.remove(&client_id);

        warn!("Proxy has shuted down");

        Ok(Response::new(ProxyShutdownResponse { success: true }))
    }
}
