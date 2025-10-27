use crate::models::servers::Client;
use tonic::{Request, Response, Status};
use tracing::warn;

use crate::{
    BridgeService,
    bridge::{ProxyShutdownRequest, ProxyShutdownResponse},
};

impl BridgeService {
    pub async fn handle_proxy_shutdown(
        &self,
        request: Request<ProxyShutdownRequest>,
    ) -> Result<Response<ProxyShutdownResponse>, Status> {
        let inner_request = request.into_inner();
        let client_id = inner_request.client_id;

        warn!("Proxy shutdown request from client {} received", client_id);

        let client = Client::get(&self.cache.servers.proxies, client_id).await?;

        if !client {
            return Err(Status::unavailable("Proxy is not active"));
        }

        self.cache.servers.proxies.remove(&client_id);

        warn!("Proxy shutdown request from client {} completed", client_id);

        Ok(Response::new(ProxyShutdownResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use crate::BridgeService;
    use crate::bridge::{ProxyShutdownRequest, ProxyStartupRequest};
    use crate::logger::Logger;
    use tonic::Request;

    #[tokio::test]
    async fn test_handle_proxy_shutdown() {
        Logger::init(true).await;
        let service = BridgeService::new().await;

        let s_req = Request::new(ProxyStartupRequest {});

        let s_resp = service
            .handle_proxy_startup(s_req)
            .await
            .unwrap()
            .into_inner();

        let client_id = s_resp.client_id;

        assert!(service.cache.servers.proxies.contains_key(&client_id));

        let sh_req = Request::new(ProxyShutdownRequest { client_id });

        let s_resp = service
            .handle_proxy_shutdown(sh_req)
            .await
            .unwrap()
            .into_inner();

        assert!(s_resp.success);
        assert!(!service.cache.servers.proxies.contains_key(&client_id));
    }
}
