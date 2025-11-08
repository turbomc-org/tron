use crate::BridgeService;
use crate::bridge::{ProxyConnectionResponse, ServerOffline};

impl BridgeService {
    pub async fn server_offline(&self, msg: MessagePlayer) {
        let mut dead_clients = Vec::new();

        for entry in self.state().proxy_connections.iter() {
            let client_id = *entry.key();
            let tx = entry.value();

            let response = ProxyConnectionResponse {
                command: Some(crate::bridge::proxy_connection_response::Command::Message(
                    msg.clone(),
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
