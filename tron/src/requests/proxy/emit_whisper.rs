use crate::BridgeService;
use crate::config::messages::{
    FAILED_TO_SEND_WHISPER, NOT_A_FRIEND, PLAYER_OFFLINE, TARGET_NOT_FOUND,
};
use crate::render;
use crate::utils::format_message::format_message;
use tracing::{error, warn};
use tron_protos::EmitWhisper;

impl BridgeService {
    pub async fn handle_proxy_emit_whisper(&self, req: EmitWhisper) {
        let username = req.username;
        let target_username = req.target;
        let message = req.message;

        let player = match self.state().get_player_with_handling(&username).await {
            Ok(player) => player,
            Err(err) => {
                error!("Failed to get player '{}': {}", username, err);
                return;
            }
        };

        let target = match self
            .state()
            .get_player_with_handling(&target_username)
            .await
        {
            Ok(target) => target,
            Err(err) => {
                error!("Failed to get target player '{}': {}", target_username, err);
                let _ = self
                    .send_message(
                        &username,
                        render!(TARGET_NOT_FOUND, target = &target_username),
                    )
                    .await;
                return;
            }
        };

        let is_friend = player.friends.contains(&target.id) && target.friends.contains(&player.id);
        if !is_friend {
            warn!(
                "Player '{}' tried to whisper to non-friend '{}'",
                username, target_username
            );
            let _ = self
                .send_message(&username, render!(NOT_A_FRIEND, target = &target.username))
                .await;
            return;
        }

        let formatted = match format_message(&self.state(), &player, &message).await {
            Ok(msg) => msg,
            Err(err) => {
                error!(
                    "Failed to format whisper message for '{}': {}",
                    username, err
                );
                if let Err(e) = self
                    .send_message(
                        &username,
                        render!(FAILED_TO_SEND_WHISPER, target = &target_username),
                    )
                    .await
                {
                    error!(
                        "Failed to send failed whisper message to '{}': {}",
                        username, e
                    );
                };
                return;
            }
        };

        if self
            .send_message(&target.username, formatted)
            .await
            .is_err()
        {
            error!("Failed to deliver whisper to '{}'", target.username);
            if let Err(e) = self
                .send_message(
                    &username,
                    render!(PLAYER_OFFLINE, username = &target.username),
                )
                .await
            {
                error!("Failed to send offline message to '{}': {}", username, e);
            };
            return;
        }

        let confirm_msg = format!("[To {}] {}", target.username, message);
        if let Err(e) = self.send_message(&username, confirm_msg).await {
            error!(
                "Failed to send confirmation message to '{}': {}",
                username, e
            );
        };
    }
}
