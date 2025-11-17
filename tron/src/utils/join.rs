use crate::BridgeService;
use crate::bridge::{ProxyServer, TransferPlayer};
use crate::config::messages::{SUCCESSFUL_LOGIN, TRANSFERRING_PLAYER};
use crate::models::player::Player;
use crate::render;
use tracing::error;
use tracing::info;

impl BridgeService {
    pub async fn join_game(&self, player: Player) {
        let username = player.username.clone();

        if let Err(e) = self
            .send_message(&username, render!(SUCCESSFUL_LOGIN, username = username))
            .await
        {
            error!("Failed to send player message: {}", e);
        }

        let landing_opt = self.state().servers.landing.lock().unwrap().clone();

        info!(landing_opt);

        if let Some(landing_id) = landing_opt {
            if let Err(e) = self
                .send_message(&username, render!(TRANSFERRING_PLAYER, username = username))
                .await
            {
                error!("Failed to send TRANSFERRING_PLAYER to {}: {}", username, e);
            }

            let server = match self.state().servers.documents.get(&landing_id) {
                Some(s) => s.clone(),
                None => {
                    error!("Landing server ID {} not found in documents", landing_id);
                    return;
                }
            };

            // todo make transfer player
        } else {
            info!(
                "No landing server configured (landing = None), keeping player {} on the proxy",
                username
            );
        }
    }
}
