use tracing::info;

use crate::BridgeService;
use crate::bridge::Heartbeat;

impl BridgeService {
    pub async fn handle_proxy_heartbeat(&self, _req: Heartbeat) {
        info!("ITS A HEARTBEAT");
    }
}
