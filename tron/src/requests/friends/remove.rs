use crate::BridgeService;
use crate::bridge::{RemoveFriendRequest, RemoveFriendResponse};
use tonic::{Request, Response, Status};
use tracing::{debug, error};

impl BridgeService {
    pub async fn handle_remove_friend(
        &self,
        request: Request<RemoveFriendRequest>,
    ) -> Result<Response<RemoveFriendResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        debug!("Remove friend request from player {} received", username);

        if username == target {
            return Err(Status::invalid_argument(
                "Cannot remove yourself from your friend list",
            ));
        }

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let target_id = self.cache.get_friend_id(&player, &target).await?;

        player
            .remove_friend(
                target_id,
                &self.databases.players,
                &self.cache.active_players,
                &self.cache.player_indexes,
            )
            .await
            .map_err(|err| {
                error!(
                    "Failed to remove player {} from player {} friend list: {}",
                    target,
                    username,
                    err.to_string()
                );

                Status::internal(format!(
                    "Failed to remove player {} from your friend list",
                    target
                ))
            })?;

        Ok(Response::new(RemoveFriendResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use crate::BridgeService;
    use crate::bridge::bridge_server::Bridge;
    use crate::logger::Logger;
    use mongodb::bson::doc;

    #[tokio::test]
    async fn test_remove_friend() {
        // --- Setup (same as before) ---
        Logger::init(true).await;
        let service = BridgeService::new().await;

        let username = "player_remove_1".to_string();
        let friend = "player_remove_2".to_string();
        let edition = 1;

        service
            .player_join(tonic::Request::new(crate::bridge::PlayerJoinRequest {
                username: username.clone(),
                edition,
            }))
            .await
            .unwrap();

        service
            .player_join(tonic::Request::new(crate::bridge::PlayerJoinRequest {
                username: friend.clone(),
                edition,
            }))
            .await
            .unwrap();

        service
            .send_friend_request(tonic::Request::new(
                crate::bridge::SendFriendRequestRequest {
                    sender: username.clone(),
                    receiver: friend.clone(),
                },
            ))
            .await
            .unwrap();

        service
            .accept_friend_request(tonic::Request::new(
                crate::bridge::AcceptFriendRequestRequest {
                    username: friend.clone(),
                    sender: username.clone(),
                },
            ))
            .await
            .unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

        let remove_req = tonic::Request::new(crate::bridge::RemoveFriendRequest {
            username: username.clone(),
            target: friend.clone(),
        });
        let remove_resp = service
            .remove_friend(remove_req)
            .await
            .unwrap()
            .into_inner();
        assert!(remove_resp.success);

        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

        let cached_player1 = service.cache.active_players.get(&username).unwrap();
        let cached_player2 = service.cache.active_players.get(&friend).unwrap();
        let player2_id = cached_player2.id;

        assert!(
            !cached_player1.friends.contains(&player2_id),
            "Friend should be removed from player 1's cache immediately."
        );

        service
            .databases
            .players
            .delete_one(doc! {"username": &username})
            .await
            .unwrap();
        service
            .databases
            .players
            .delete_one(doc! {"username": &friend})
            .await
            .unwrap();
    }
}
