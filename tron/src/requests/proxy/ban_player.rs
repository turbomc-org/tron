use crate::BridgeService;
use tron_protos::{BanPlayer, ProxyConnectionResponse};

impl BridgeService {
    pub async fn ban_player(&self, req: BanPlayer) {
        let mut dead_clients = Vec::new();

        for entry in self.state().proxy_connections.iter() {
            let client_id = *entry.key();
            let tx = entry.value();

            let response = ProxyConnectionResponse {
                command: Some(tron_protos::proxy_connection_response::Command::Ban(
                    req.clone(),
                )),
            };

            if tx.send(Ok(response)).await.is_err() {
                dead_clients.push(client_id);
            }
        }

        for id in dead_clients {
            self.state().proxy_connections.remove(&id);
        }
    }
}
