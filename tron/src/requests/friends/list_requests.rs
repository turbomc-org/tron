use crate::bridge::{ListFriendRequestsRequest, ListFriendRequestsResponse};
use crate::config::messages::{INCOMING_FRIEND_REQUESTS, NO_INCOMING_FRIEND_REQUESTS};
use crate::{BridgeService, render};
use chrono::Utc;
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_list_friend_requests(
        &self,
        request: Request<ListFriendRequestsRequest>,
    ) -> Result<Response<ListFriendRequestsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username.clone();

        info!("Get friend requests for player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        // Resolve all incoming friend requests synchronously
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
                render!(NO_INCOMING_FRIEND_REQUESTS, username = &player.username),
            )
            .await
            .map_err(|err| {
                error!("Failed to send player message: {}", err);
            })
            .unwrap();
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
                render!(
                    INCOMING_FRIEND_REQUESTS,
                    count = &count,
                    s = if count == 1 { "" } else { "s" },
                    list = &list
                ),
            )
            .await
            .map_err(|err| {
                error!("Failed to send player message: {}", err);
            })
            .unwrap();
        }

        info!("Get friend requests for player {} completed", username);

        Ok(Response::new(ListFriendRequestsResponse {
            incoming_friend_requests,
        }))
    }
}
