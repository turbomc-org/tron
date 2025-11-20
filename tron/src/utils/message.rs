use std::collections::HashSet;

use crate::BridgeService;
use anyhow::Result;
use tracing::error;
use tron_protos::MessagePlayer;
pub struct Message {}

impl BridgeService {
    pub async fn send_message(&self, username: &str, message: String) {
        if let Err(e) = self
            .message_player(MessagePlayer {
                username: username.to_string(),
                message: message.to_string(),
            })
            .await
        {
            error!("Failed to send message to player {}: {}", username, e);
        }
    }

    pub async fn send_message_to_admins(&self, message: String) -> Result<()> {
        let admin_names: HashSet<String> = self
            .state()
            .indexes
            .admin
            .iter()
            .map(|id| {
                self.state()
                    .get_player_username(&id)
                    .unwrap_or_else(|| "Unknown".to_string())
            })
            .collect();

        let active_admins: Vec<String> = self
            .state()
            .active_players
            .iter()
            .filter(|player| admin_names.contains(&player.value().username))
            .map(|player| player.value().username.clone())
            .collect();

        for admin_username in active_admins {
            self.send_message(&admin_username, message.clone()).await;
        }

        Ok(())
    }

    pub async fn send_message_to_moderators(&self, message: String) -> Result<()> {
        let moderator_names: HashSet<String> = self
            .state()
            .indexes
            .moderator
            .iter()
            .map(|id| {
                self.state()
                    .get_player_username(&id.clone())
                    .unwrap_or_else(|| "Unknown".to_string())
            })
            .collect();

        let active_moderators: Vec<String> = self
            .state()
            .active_players
            .iter()
            .filter(|player| moderator_names.contains(&player.value().username))
            .map(|player| player.value().username.clone())
            .collect();

        for moderator in active_moderators {
            self.send_message(&moderator, message.clone()).await;
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! message {
    (
        type: $type:literal,
        title: $title:literal,
        body: $body:literal
        $(, actions: { $(
            $label:ident : {
                kind: $kind:literal,
                value: $val:literal
                $(, label: $text:literal)?
            }
        ),* $(,)? })?
        $(,)?
    ) => {{
        use text_placeholder::Template;

        let (header_grad, accent_grad) = match $type {
            "error" => ("#FF4D4D:#FF0000", "#FF4D4D:#FF0000"),
            "warning" => ("#FFD700:#FF8C00", "#FFB300:#FF7300"),
            "success" => ("#C724B1:#7A00FF", "#B200FF:#6A00A3"),
            "system" => ("#00BFFF:#0077FF", "#80D4FF:#0088FF"),
            _ => ("#C724B1:#7A00FF", "#B200FF:#6A00A3"),
        };

        let mut msg = format!(
            "<gradient:{grad}><bold>{title}</bold></gradient>\n\
             <gray>{body}</gray>",
            grad = header_grad,
            title = $title,
            body = $body
        );

        $(
            $(
                let mut action_label = String::new();
                $( action_label.push_str($text); )? // only expands if label: "..." was present
                if action_label.is_empty() {
                    action_label = stringify!($label).replace('_', " ").to_uppercase();
                }

                // build opening click tag (we'll always close with </click>)
                let open = if $kind == "url" {
                    format!("click:open_url:'{}'", $val)
                } else {
                    format!("click:run_command:'{}'", $val)
                };

                // append the action line
                // use a small local format to avoid complexity with named/positional mixes
                let action_html = format!(
                    "\n<dark_gray>»</dark_gray> <{open}><u><gradient:{accent}>{}</gradient></u></click>",
                    action_label,
                    open = open,
                    accent = accent_grad
                );

                msg.push_str(&action_html);
            )*
        )?

        // Leak msg into 'static for Lazy<Template<'static>> usage
        let static_ref: &'static str = Box::leak(msg.into_boxed_str());
        Template::new(static_ref)
    }};
}
