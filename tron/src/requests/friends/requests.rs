use crate::BridgeService;
use chrono::Utc;
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{GetFriendRequestsRequest, GetFriendRequestsResponse};

impl BridgeService {
    pub async fn handle_get_friend_requests(
        &self,
        request: Request<GetFriendRequestsRequest>,
    ) -> Result<Response<GetFriendRequestsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username.clone();

        info!("Get friend requests for player {} received", username);

        let player = self.player(&username).await?;

        let mut incoming_friend_requests: HashMap<String, u64> = HashMap::new();

        for (sender_id, sent_at) in &player.incoming_friend_requests {
            if let Some(sender_name) = self.state().get_player_username(sender_id) {
                incoming_friend_requests.insert(sender_name, *sent_at);
            }
        }

        let count = incoming_friend_requests.len();

        if count == 0 {
            self.send_message(
                &player.username,
                "<gradient:#C724B1:#7A00FF><bold>📭 NO INCOMING FRIEND REQUESTS</bold></gradient>\n\
                 <gray>Your <gradient:#B200FF:#6A00A3>Turbo Network</gradient> inbox is currently empty.</gray>\n\
                 <dark_gray>»</dark_gray> <click:run_command:'/friends'><u><gradient:#C724B1:#7A00FF>Send a new request</gradient></u></click>"
                    .to_string(),
            )
            .await;
        } else {
            let now = Utc::now().timestamp() as u64;
            let mut entries = Vec::new();

            for (sender_name, sent_at) in &incoming_friend_requests {
                let elapsed_secs = now.saturating_sub(*sent_at);
                let time_ago = if elapsed_secs < 60 {
                    "just now".to_string()
                } else if elapsed_secs < 3600 {
                    format!("{} minute(s) ago", elapsed_secs / 60)
                } else if elapsed_secs < 86400 {
                    format!("{} hour(s) ago", elapsed_secs / 3600)
                } else {
                    format!("{} day(s) ago", elapsed_secs / 86400)
                };

                entries.push(format!(
                    "<dark_gray>»</dark_gray> <light_purple><bold>{}</bold></light_purple> \
                     <gray>— sent {}</gray>",
                    sender_name, time_ago
                ));
            }

            let list = entries.join("\n");

            self.send_message(
                &player.username,
                format!(
                    "<gradient:#C724B1:#7A00FF><bold>📨 INCOMING FRIEND REQUESTS</bold></gradient>\n\
                     <gray>You have <light_purple><bold>{}</bold></light_purple> pending connection{} \
                     on the <gradient:#B200FF:#6A00A3>Turbo Network</gradient>.</gray>\n\
                     {}\n\
                     <dark_gray>»</dark_gray> <click:run_command:'/friends accept_all'>\
                     <u><gradient:#C724B1:#7A00FF>[ ACCEPT ALL ]</gradient></u></click>  \
                     <click:run_command:'/friends deny_all'>\
                     <u><gradient:#8A2BE2:#C724B1>[ DENY ALL ]</gradient></u></click>",
                    count,
                    if count == 1 { "" } else { "s" },
                    list
                ),
            )
            .await;
        }

        info!("Get friend requests for player {} completed", username);

        Ok(Response::new(GetFriendRequestsResponse {
            incoming_friend_requests,
        }))
    }
}
