use tracing::info;

use crate::BridgeService;
use crate::bridge::Heartbeat;

impl BridgeService {
    pub async fn handle_proxy_heartbeat(&self, request: Heartbeat) {
        let players = request.players;

        let active_players = self.state().active_players.clone();

        active_players.retain(|username, _record| {
            let still_online = players.contains(username);

            if !still_online {
                info!(
                    "Player {} removed from active state (not in heartbeat)",
                    username
                );
            }

            still_online
        });
    }
}
