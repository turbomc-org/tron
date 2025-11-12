use tonic::{Request, Response, Status};

use crate::{
    BridgeService,
    bridge::{ProxyShutdownRequest, ProxyShutdownResponse},
};

impl BridgeService {
    pub async fn handle_proxy_shutdown(
        &self,
        request: Request<ProxyShutdownRequest>,
    ) -> Result<Response<ProxyShutdownResponse>, Status> {
        todo!("Implement proxy shutdown handling")
    }
}
