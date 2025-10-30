use crate::bridge::{PlayerKillRequest, PlayerKillResponse};
use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::{debug, error};

use crate::BridgeService;

impl BridgeService {
    #[tracing::instrument]
    pub async fn handle_player_kill(
        &self,
        request: Request<PlayerKillRequest>,
    ) -> Result<Response<PlayerKillResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Kill request of player {} received", username);

        let mut player = self.state.get_player_with_handling(&username).await?;

        player
            .add_kill(1, &self.collections.players, &self.state)
            .await
            .map_err(|err| {
                error!("Failed to add death of player {}: {}", username, err);
                Status::internal("Failed to add death")
            })?;

        debug!("Kill request of player {} completed", username);

        Ok(Response::new(PlayerKillResponse { success: true }))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::BridgeService;
//     use crate::bridge::bridge_server::Bridge;
//     use crate::logger::Logger;
//     use mongodb::bson::doc;
//     use tonic::Request;

//     #[tokio::test]
//     async fn test_player_kill() {
//         // Initialize logger and service
//         Logger::init(true).await;
//         let service = BridgeService::new().await;

//         // Create player
//         let username = "player_kill_1".to_string();
//         let edition = 1;

//         // Register player
//         let player_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
//             username: username.clone(),
//             edition,
//         });
//         let player_resp = service.player_join(player_req).await.unwrap().into_inner();
//         assert!(player_resp.success, "Player should join successfully");

//         // Fetch player record before kill
//         let before_record = service
//             .databases
//             .players
//             .find_one(doc! {"username": &username})
//             .await
//             .unwrap()
//             .unwrap();
//         let before_kills = before_record.kills;

//         // Trigger player kill
//         let kill_req = Request::new(crate::bridge::PlayerKillRequest {
//             username: username.clone(),
//         });
//         let kill_resp = service.player_kill(kill_req).await.unwrap().into_inner();

//         assert!(kill_resp.success, "Player kill request should succeed");

//         // Wait for async update task to complete
//         tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

//         // Fetch player record after kill
//         let after_record = service
//             .databases
//             .players
//             .find_one(doc! {"username": &username})
//             .await
//             .unwrap()
//             .unwrap();
//         let after_kills = after_record.kills;

//         // Verify kills incremented by 1
//         assert_eq!(
//             after_kills,
//             before_kills + 1,
//             "Player kills should increase by 1 after kill"
//         );

//         // Verify cache updated
//         let cached_player = service.cache.get_player(&username).await.unwrap().unwrap();
//         assert_eq!(
//             cached_player.kills, after_kills,
//             "Cache should reflect the latest kill count"
//         );

//         // Cleanup
//         service
//             .databases
//             .players
//             .delete_one(doc! {"username": &username})
//             .await
//             .unwrap();
//     }
// }
