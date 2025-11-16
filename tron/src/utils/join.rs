use crate::BridgeService;
use crate::bridge::{ProxyServer, TransferPlayer};
use crate::config::messages::{SUCCESSFUL_LOGIN, TRANSFERRING_PLAYER};
use crate::models::player::Player;
use crate::render;
use tracing::error;

impl BridgeService {
    pub async fn join_game(&self, player: Player) {
        let username = player.username.clone();

        if let Err(e) = self
            .send_message(&username, render!(SUCCESSFUL_LOGIN, username = username))
            .await
        {
            error!("Failed to send player message: {}", e);
        }

        if let Err(e) = self
            .send_message(&username, render!(TRANSFERRING_PLAYER, username = username))
            .await
        {
            error!("Failed to send player message: {}", e);
        }

        let landing_id = self.state().servers.landing.clone();

        let id = landing_id.lock().unwrap().expect("landing_id not set");

        let server = match self.state().servers.documents.get(&id) {
            Some(server) => server.clone(),
            None => {
                error!("Failed to find server with id {}", id);
                return;
            }
        };

        self.transfer_player(TransferPlayer {
            username: username,
            server: Some(ProxyServer {
                id: server.id,
                address: server.address.clone(),
                name: server.name,
                motd: server.description,
            }),
        })
        .await;
    }
}
