use crate::BridgeService;
use crate::bridge::{AcceptFriendRequestRequest, AcceptFriendRequestResponse};
use crate::models::player::Player;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    pub async fn handle_accept_friend_request(
        &self,
        request: Request<AcceptFriendRequestRequest>,
    ) -> Result<Response<AcceptFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Accept friend request from {} received", username);

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let sender = inner_request.sender;
        let players = &self.databases.players.clone();

        debug!(
            "Checking if {} has a friend request from {}",
            username, sender
        );
        let sender_id = self.cache.check_friend_request(&player, &sender).await?;

        Player::accept_friend_request(player.id, sender_id, players)
            .await
            .map_err(|err| {
                error!("Failed to accept friend request from {}: {}", sender, err);

                Status::internal(format!("Failed to accept friend request from {}", sender))
            })?;

        player.incoming_friend_requests.remove(&sender_id);
        player.friends.insert(sender_id);
        self.cache.insert_player(player).await.map_err(|err| {
            error!(
                "Failed to update active player {} in cache: {}",
                username, err
            );
            Status::internal(format!(
                "Failed to update active player {} in cache",
                username
            ))
        })?;

        info!("Accept friend request from {} response", username);

        Ok(Response::new(AcceptFriendRequestResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use crate::BridgeService;
    use crate::bridge::bridge_server::Bridge;
    use crate::logger::Logger;
    use mongodb::bson::doc;

    #[tokio::test]
    async fn test_accept_friend_request() {
        Logger::init(true).await;

        let service = BridgeService::new().await;

        let username = "vaibhav_57891".to_string();
        let friend = "biharini_57891".to_string();
        let edition = 1;

        let player_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
            username: username.clone(),
            edition,
        });

        let player_resp = service.player_join(player_req).await.unwrap().into_inner();

        assert!(player_resp.success);

        let friend_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
            username: friend.clone(),
            edition,
        });

        let friend_resp = service.player_join(friend_req).await.unwrap().into_inner();

        assert!(friend_resp.success);

        let friend_request_req = tonic::Request::new(crate::bridge::SendFriendRequestRequest {
            sender: username.clone(),
            receiver: friend.clone(),
        });

        let friend_req_resp = service
            .send_friend_request(friend_request_req)
            .await
            .unwrap()
            .into_inner();

        assert!(friend_req_resp.success);

        let accept_friend_request_req =
            tonic::Request::new(crate::bridge::AcceptFriendRequestRequest {
                username: friend.clone(),
                sender: username.clone(),
            });

        let accept_friend_request_resp = service
            .accept_friend_request(accept_friend_request_req)
            .await
            .unwrap()
            .into_inner();

        assert!(accept_friend_request_resp.success);

        let sender_record = service
            .databases
            .players
            .find_one(doc! {"username": &username})
            .await
            .unwrap()
            .unwrap();

        let receiver_record = service
            .databases
            .players
            .find_one(doc! {"username": &friend})
            .await
            .unwrap()
            .unwrap();

        let is_a_friend = sender_record.friends.contains(&receiver_record.id)
            && receiver_record.friends.contains(&sender_record.id);

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

        assert!(is_a_friend)
    }
}
