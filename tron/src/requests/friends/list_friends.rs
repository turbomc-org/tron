use crate::config::messages::{FRIEND_NETWORK, NO_CONNECTIONS};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{ListFriendsRequest, ListFriendsResponse};

impl BridgeService {
    pub async fn handle_list_friends(
        &self,
        request: Request<ListFriendsRequest>,
    ) -> Result<Response<ListFriendsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username.clone();

        info!("Get friends list for player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        let mut friends: Vec<String> = Vec::new();
        for friend_id in &player.friends {
            if let Some(friend_name) = self.state().get_player_username(friend_id) {
                friends.push(friend_name);
            }
        }

        if friends.is_empty() {
            self.send_message(
                &username,
                render!(NO_CONNECTIONS, username = &player.username),
            )
            .await
            .map_err(|err| error!("Failed to send player message: {}", err))
            .ok();
        } else {
            let mut friend_entries = Vec::new();

            for friend_name in &friends {
                let is_online = self.state().active_players.contains_key(friend_name);
                let status_icon = if is_online {
                    "<green>●</green>"
                } else {
                    "<dark_gray>●</dark_gray>"
                };

                friend_entries.push(format!(
                    "<dark_gray>»</dark_gray> {} <white><bold>{}</bold></white> \
                     <gray>-</gray> {} [<red><click:run_command:'/friend remove {}'>Remove</click></red>]",
                    status_icon,
                    friend_name,
                    if is_online { "<green>Online</green>" } else { "<gray>Offline</gray>" },
                    friend_name
                ));
            }

            let friend_list_str = friend_entries.join("\n");

            self.send_message(
                &username,
                render!(
                    FRIEND_NETWORK,
                    count = &friends.len(),
                    friend_list = &friend_list_str
                ),
            )
            .await
            .map_err(|err| error!("Failed to send player message: {}", err))
            .ok();
        }

        info!("Get friends list for player {} completed", username);

        Ok(Response::new(ListFriendsResponse { friends }))
    }
}
