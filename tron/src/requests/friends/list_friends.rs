use crate::bridge::{ListFriendsRequest, ListFriendsResponse};
use crate::config::messages::{FRIEND_NETWORK, NO_CONNECTIONS};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::debug;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_list_friends(
        &self,
        request: Request<ListFriendsRequest>,
    ) -> Result<Response<ListFriendsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username.clone();

        debug!("Get friends request for player {} received", username);

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
            .await;
        } else {
            let friend_list_str = friends
                .iter()
                .map(|name| {
                    let is_online = self.state().active_players.contains_key(name);
                    if is_online {
                        format!(
                            "<dark_gray> - <green>●</green> <white>{}</white></dark_gray>",
                            name
                        )
                    } else {
                        format!(
                            "<dark_gray> - <dark_gray>●</dark_gray> <gray>{}</gray></dark_gray>",
                            name
                        )
                    }
                })
                .collect::<Vec<String>>()
                .join("\n");

            self.send_message(
                &username,
                render!(
                    FRIEND_NETWORK,
                    count = &friends.len(),
                    friend_list = &friend_list_str
                ),
            )
            .await;
        }

        debug!("Get friends request for player {} completed", username);

        Ok(Response::new(ListFriendsResponse { friends }))
    }
}
