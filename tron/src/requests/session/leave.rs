use crate::BridgeService;
use crate::bridge::{PlayerLeaveRequest, PlayerLeaveResponse};
use tonic::{Request, Response, Status};
use tracing::{debug, info};

impl BridgeService {
    pub async fn handle_player_leave(
        &self,
        request: Request<PlayerLeaveRequest>,
    ) -> Result<Response<PlayerLeaveResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Leave request for player {} received", username);

        let player = self.state.get_player_with_handling(&username).await?;
        self.state.active_players.remove(&player.username);

        info!("Player {} left the game", username);

        Ok(Response::new(PlayerLeaveResponse { success: true }))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::BridgeService;
//     use crate::logger::Logger;
//     use crate::{bridge::bridge_server::Bridge, models::player::Edition};
//     use mongodb::bson::doc;

//     #[tokio::test]
//     async fn test_leave_player() {
//         Logger::init(true).await;

//         let service = BridgeService::new().await;

//         let username = "vaibhav_57889".to_string();
//         let edition = 1;

//         let req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
//             username: username.clone(),
//             edition,
//         });

//         let resp = service.player_join(req).await.unwrap().into_inner();

//         assert!(resp.success);

//         let player = service.cache.get_player(&username).await.unwrap().unwrap();

//         service
//             .databases
//             .players
//             .delete_one(doc! {"username": &username})
//             .await
//             .unwrap();

//         assert_eq!(player.edition, Edition::Java);

//         let req = tonic::Request::new(crate::bridge::PlayerLeaveRequest {
//             username: username.clone(),
//         });

//         let resp = service.player_leave(req).await.unwrap().into_inner();

//         assert!(resp.success);

//         let player = service.cache.get_player(&username).await.unwrap();

//         assert!(player.is_none());
//     }
// }

// #[tokio::test]
// async fn test_join_new_player() {
//     let mut mock_repo = MockPlayerRepository::new();

//     // Step 1: no player found
//     mock_repo.expect_find_by_username().returning(|_| Ok(None));

//     // Step 2: insert succeeds
//     mock_repo.expect_insert_player().returning(|_| Ok(()));

//     let service = BridgeService::new(Arc::new(mock_repo));

//     let req = Request::new(PlayerJoinRequest {
//         username: "new_user".to_string(),
//         edition: 0,
//     });

//     let resp = service.handle_player_join(req).await.unwrap().into_inner();
//     assert!(resp.success);
// }

// #[tokio::test]
// async fn test_existing_player() {
//     let mut mock_repo = MockPlayerRepository::new();

//     // Simulate found player
//     mock_repo.expect_find_by_username().returning(|_| {
//         Ok(Some(Player::new(
//             "existing_user".to_string(),
//             Edition::Java,
//         )))
//     });

//     let service = BridgeService::new(Arc::new(mock_repo));

//     let req = Request::new(PlayerJoinRequest {
//         username: "existing_user".into(),
//         edition: 0,
//     });

//     let resp = service.handle_player_join(req).await.unwrap().into_inner();
//     assert!(resp.success);
// }
