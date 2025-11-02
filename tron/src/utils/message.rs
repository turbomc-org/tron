use chrono::Utc;

use crate::{BridgeService, bridge::ServerSendMessageResponse};
pub struct Message {}

impl BridgeService {
    pub async fn send_warning_to_player(&self, username: &str, message: &str) {
        self.send_message_to_player(username, format!(
            "<color:#ff6e0d><st>[</st>WARNING<st>]</st></color> <color:#ff8b2b><i>{}<i></color>",
            message
        ))
        .await;
    }

    pub async fn send_info_to_player(&self, username: &str, message: &str) {
        self.send_message_to_player(
            username,
            format!(
                "<color:#00ff00><st>[</st>INFO<st>]</st></color> <color:#57ff57><i>{}<i></color>",
                message
            ),
        )
        .await;
    }

    pub async fn send_notification_to_player(&self, username: &str, message: &str) {
        self.send_message_to_player(username, format!(
            "<color:#12b0ff><st>[</st>NOTIFICATION<st>]</st></color> <color:#008dd4><i>{}<i></color>",
            message
        ))
        .await;
    }

    pub async fn send_message_to_player(&self, username: &str, message: String) {
        let now = Utc::now().timestamp() as u64;

        self.broadcast_message(ServerSendMessageResponse {
            username: username.to_string(),
            message: message.to_string(),
            timestamp: now,
        })
        .await
    }
}
