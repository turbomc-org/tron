use crate::bridge::{PlayerDeathRequest, PlayerDeathResponse};
use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::error;

use crate::BridgeService;

impl BridgeService {
    pub async fn handle_player_death(
        &self,
        request: Request<PlayerDeathRequest>,
    ) -> Result<Response<PlayerDeathResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let mut player = self.state.get_player_with_handling(&username).await?;

        player
            .add_death(1, &self.collections.players, &self.state)
            .await
            .map_err(|err| {
                error!("Failed to add death of player {}: {}", username, err);
                Status::internal("Failed to add death")
            })?;

        Ok(Response::new(PlayerDeathResponse { success: true }))
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
//     async fn test_player_death() {
//         Logger::init(true).await;
//         let service = BridgeService::new().await;

//         let username = "player_death_1".to_string();
//         let edition = 1;

//         let player_req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
//             username: username.clone(),
//             edition,
//         });
//         let player_resp = service.player_join(player_req).await.unwrap().into_inner();
//         assert!(player_resp.success, "Player should join successfully");

//         let before_record = service
//             .databases
//             .players
//             .find_one(doc! {"username": &username})
//             .await
//             .unwrap()
//             .unwrap();
//         let before_deaths = before_record.deaths;

//         let death_req = Request::new(crate::bridge::PlayerDeathRequest {
//             username: username.clone(),
//         });
//         let death_resp = service.player_death(death_req).await.unwrap().into_inner();

//         assert!(death_resp.success, "Player death request should succeed");

//         tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

//         let after_record = service
//             .databases
//             .players
//             .find_one(doc! {"username": &username})
//             .await
//             .unwrap()
//             .unwrap();
//         let after_deaths = after_record.deaths;

//         assert_eq!(
//             after_deaths,
//             before_deaths + 1,
//             "Player deaths should increase by 1 after death"
//         );

//         let cached_player = service.cache.get_player(&username).await.unwrap().unwrap();
//         assert_eq!(
//             cached_player.deaths, after_deaths,
//             "Cache should reflect the latest death count"
//         );

//         service
//             .databases
//             .players
//             .delete_one(doc! {"username": &username})
//             .await
//             .unwrap();
//     }
// }
