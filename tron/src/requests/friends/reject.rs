use crate::BridgeService;
use crate::bridge::{RejectFriendRequestRequest, RejectFriendRequestResponse};
use crate::models::player::Player;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_reject_friend_request(
        &self,
        request: Request<RejectFriendRequestRequest>,
    ) -> Result<Response<RejectFriendRequestResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let sender = inner_request.sender;

        info!("Reject friend request from player {} received", username);

        if username == sender {
            return Err(Status::invalid_argument(
                "Cannot reject friend request of yourself",
            ));
        }

        let mut player = self.state.get_player_with_handling(&username).await?;
        let players = &self.collections.players.clone();
        let sender_id = self.state.check_friend_request(&player, &sender).await?;

        Player::reject_friend_request(&mut player, sender_id, &players, &self.state)
            .await
            .map_err(|err| {
                error!("Failed to reject friend request from {}: {}", sender, err);

                Status::internal(format!("Failed to reject friend request from {}", sender))
            })?;

        info!("Reject friend request from player {} completed", username);

        Ok(Response::new(RejectFriendRequestResponse { success: true }))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::BridgeService;
//     use crate::bridge::bridge_server::Bridge;
//     use crate::logger::Logger;
//     use mongodb::bson::doc;

//     #[tokio::test]
//     async fn test_reject_friend_request() {
//         // Initialize logger and service
//         Logger::init(true).await;
//         let service = BridgeService::new().await;

//         // Create two players
//         let username = "player_reject_1".to_string();
//         let friend = "player_reject_2".to_string();
//         let edition = 1;

//         // Register player 1
//         let player_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
//             username: username.clone(),
//             edition,
//         });
//         let player_resp = service.player_join(player_req).await.unwrap().into_inner();
//         assert!(player_resp.success);

//         // Register player 2
//         let friend_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
//             username: friend.clone(),
//             edition,
//         });
//         let friend_resp = service.player_join(friend_req).await.unwrap().into_inner();
//         assert!(friend_resp.success);

//         // Send friend request from player 1 to player 2
//         let send_req = tonic::Request::new(crate::bridge::SendFriendRequestRequest {
//             sender: username.clone(),
//             receiver: friend.clone(),
//         });
//         let send_resp = service
//             .send_friend_request(send_req)
//             .await
//             .unwrap()
//             .into_inner();
//         assert!(send_resp.success);

//         // Reject the friend request as player 2
//         let reject_req = tonic::Request::new(crate::bridge::RejectFriendRequestRequest {
//             username: friend.clone(),
//             sender: username.clone(),
//         });
//         let reject_resp = service
//             .reject_friend_request(reject_req)
//             .await
//             .unwrap()
//             .into_inner();

//         assert!(reject_resp.success);

//         tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

//         // Fetch updated player records from MongoDB
//         let sender_record = service
//             .databases
//             .players
//             .find_one(doc! {"username": &username})
//             .await
//             .unwrap()
//             .unwrap();

//         let receiver_record = service
//             .databases
//             .players
//             .find_one(doc! {"username": &friend})
//             .await
//             .unwrap()
//             .unwrap();

//         // Assert they are NOT friends
//         let are_friends = sender_record.friends.contains(&receiver_record.id)
//             || receiver_record.friends.contains(&sender_record.id);
//         assert!(
//             !are_friends,
//             "Players should not be friends after rejection"
//         );

//         // Also assert that the receiver no longer has a pending friend request
//         assert!(
//             !receiver_record
//                 .incoming_friend_requests
//                 .contains_key(&sender_record.id),
//             "Receiver should not have pending friend request after rejection"
//         );

//         // Cleanup database
//         service
//             .databases
//             .players
//             .delete_one(doc! {"username": &username})
//             .await
//             .unwrap();
//         service
//             .databases
//             .players
//             .delete_one(doc! {"username": &friend})
//             .await
//             .unwrap();
//     }
// }
