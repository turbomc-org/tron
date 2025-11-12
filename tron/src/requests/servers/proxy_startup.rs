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
        todo!("Implement proxy startup handling")
    }
}
