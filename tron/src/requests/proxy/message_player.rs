use crate::BridgeService;
use anyhow::Result;
use tron_protos::{MessagePlayer, ProxyConnectionResponse};

impl BridgeService {
    pub async fn message_player(&self, req: MessagePlayer) -> Result<()> {
        let mut dead_clients = Vec::new();

        for entry in self.state().proxy_connections.iter() {
            let client_id = *entry.key();
            let tx = entry.value();

            let response = ProxyConnectionResponse {
                command: Some(tron_protos::proxy_connection_response::Command::Message(
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

        Ok(())
    }
}
