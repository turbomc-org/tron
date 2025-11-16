use crate::BridgeService;
use crate::bridge::{ProxyConnectionResponse, TransferPlayer};
use tracing::info;

impl BridgeService {
    pub async fn transfer_player(&self, req: TransferPlayer) {
        info!("Transferring player {}", req.username);

        let mut dead_clients = Vec::new();

        for entry in self.state().proxy_connections.iter() {
            let client_id = *entry.key();
            let tx = entry.value();

            let response = ProxyConnectionResponse {
                command: Some(crate::bridge::proxy_connection_response::Command::Transfer(
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
