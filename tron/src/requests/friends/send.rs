use crate::BridgeService;
use crate::bridge::{SendFriendRequestRequest, SendFriendRequestResponse};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    pub async fn handle_send_friend_request(
        &self,
        request: Request<SendFriendRequestRequest>,
    ) -> Result<Response<SendFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.sender;
        let target_username = inner_request.receiver;
        let players = self.collections.players.clone();

        info!(
            "Send friend request request from player {} received",
            username
        );

        if username == target_username {
            error!(
                "Player {} tried to send a friend request to themselves",
                username
            );
            return Err(Status::invalid_argument(
                "Cannot send friend request to yourself",
            ));
        }

        let player = self.cache.get_player_with_handling(&username).await?;
        let mut target = self
            .cache
            .get_player_with_handling(&target_username)
            .await?;

        let now = Utc::now().timestamp() as u64;

        player
            .add_friend_request(
                &mut target,
                now.clone(),
                &players,
                &self.cache.active_players,
            )
            .await
            .map_err(|err| {
                error!(
                    "Failed to send friend request to {} due to {}",
                    target_username,
                    err.to_string()
                );
                Status::internal(format!(
                    "Failed to send friend request to {}",
                    target_username
                ))
            })?;

        debug!("Checking if player {} is online", target_username);
        if self.cache.active_players.contains_key(&target_username) {
            debug!("Adding request to player {}", target_username);
            self.cache
                .add_friend_request(target_username.clone(), player.id, now)
                .await?;
        }

        info!(
            "Player {} send a friend request to {}",
            username, target_username
        );

        Ok(Response::new(SendFriendRequestResponse { success: true }))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::BridgeService;
//     use crate::bridge::bridge_server::Bridge;
//     use crate::logger::Logger;
//     use mongodb::bson::doc;

//     #[tokio::test]
//     async fn test_send_friend_request() {
//         Logger::init(true).await;

//         let service = BridgeService::new().await;

//         let username = "vaibhav_57890".to_string();
//         let friend = "biharini_57809".to_string();
//         let edition = 1;

//         let player_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
//             username: username.clone(),
//             edition,
//         });

//         let player_resp = service.player_join(player_req).await.unwrap().into_inner();

//         assert!(player_resp.success);

//         let friend_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
//             username: friend.clone(),
//             edition,
//         });

//         let friend_resp = service.player_join(friend_req).await.unwrap().into_inner();

//         assert!(friend_resp.success);

//         let friend_request_req = tonic::Request::new(crate::bridge::SendFriendRequestRequest {
//             sender: username.clone(),
//             receiver: friend.clone(),
//         });

//         let friend_req_resp = service
//             .send_friend_request(friend_request_req)
//             .await
//             .unwrap()
//             .into_inner();

//         assert!(friend_req_resp.success);

//         let sender_document = service.cache.get_player(&username).await.unwrap().unwrap();
//         let receiver_document = service.cache.get_player(&friend).await.unwrap().unwrap();

//         let verification = receiver_document
//             .incoming_friend_requests
//             .contains_key(&sender_document.id);

//         assert!(verification);

//         service
//             .databases
//             .players
//             .delete_one(doc! {"username": username})
//             .await
//             .unwrap();

//         service
//             .databases
//             .players
//             .delete_one(doc! {"username": friend})
//             .await
//             .unwrap();
//     }
// }
