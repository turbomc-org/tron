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
