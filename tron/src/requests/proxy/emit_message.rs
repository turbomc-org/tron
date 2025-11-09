use crate::bridge::EmitMessage;
use crate::config::messages::NOT_SUBSCRIBED;
use crate::utils::format_message::format_message;
use crate::{BridgeService, render};
use tracing::{error, warn};

impl BridgeService {
    pub async fn handle_proxy_emit_message(&self, req: EmitMessage) {
        let username = req.username;

        let player = match self.state().get_player_with_handling(&username).await {
            Ok(player) => player,
            Err(err) => {
                error!("❌ Failed to get player '{}': {}", username, err);
                return;
            }
        };

        let message = format_message(&self.state(), &player, &req.message)
            .await
            .map_err(|err| {
                error!(
                    "Failed to format message for player '{}': {}",
                    username, err
                );
                return;
            })
            .unwrap();

        let subscription = match self.state().messaging.subscriptions.get(&player.id) {
            Some(sub) => *sub.value(),
            None => {
                warn!("⚠️ Player '{}' not subscribed to any stream", username);
                self.send_message(&username, render!(NOT_SUBSCRIBED, username = &username))
                    .await
                    .map_err(|err| {
                        error!("Failed to send player message: {}", err);
                    })
                    .unwrap();
                return;
            }
        };

        if !self.state().messaging.streams.contains(&subscription) {
            error!(
                "❌ Stream {} not found for player '{}'",
                subscription, username
            );
            self.send_message(&username, render!(NOT_SUBSCRIBED, username = &username))
                .await
                .map_err(|err| {
                    error!("Failed to send player message: {}", err);
                })
                .unwrap();
            return;
        }

        let players_in_stream = self.state().messaging.get_players_in_stream(subscription);
        if players_in_stream.is_empty() {
            warn!("⚠️ No other players subscribed to stream {}", subscription);
            return;
        }

        for player_id in players_in_stream {
            let Some(username) = self.state().indexes.player.get(&player_id) else {
                warn!("⚠️ Missing username for player ID {}", player_id);
                continue;
            };

            if let Err(e) = self.send_message(&username, message.clone()).await {
                error!(
                    "❌ Failed to send message from '{}' to '{}': {:?}",
                    player.username,
                    username.clone(),
                    e
                );
            }
        }
    }
}
