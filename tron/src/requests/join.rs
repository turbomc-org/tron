use crate::BridgeService;
use crate::bridge::{PlayerJoinRequest, PlayerJoinResponse};
use crate::models::player::Edition;
use crate::models::player::Player;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    pub async fn handle_player_join(
        &self,
        request: Request<PlayerJoinRequest>,
    ) -> Result<Response<PlayerJoinResponse>, Status> {
        let players = self.databases.players.clone();
        let inner_request = request.into_inner();
        let username = inner_request.username;
        #[allow(deprecated)]
        let edition: Edition =
            crate::bridge::player_join_request::Edition::from_i32(inner_request.edition)
                .unwrap_or(crate::bridge::player_join_request::Edition::Java)
                .into();
        let players_cache = &self.cache.active_players.clone();

        debug!("Join request for player {} received", username);

        if players_cache.contains_key(&username) {
            debug!("Player {} already exists in cache", username);
            return Ok(Response::new(PlayerJoinResponse { success: false }));
        }

        debug!("Fetching player {} record from mongodb", username);

        let response = match Player::find_by_username(username.clone(), &players).await {
            Ok(Some(player)) => {
                debug!("Inserting player {} into cache", username);

                self.cache.insert_player(player).await.map_err(|err| {
                    error!("Failed to insert player into cache: {}", err);
                    Status::internal(format!("Failed to insert player into cache: {}", err))
                })?;

                debug!("Successfully inserted player {} into cache", username);

                info!("Player {} joined the server", username);
                Response::new(PlayerJoinResponse { success: true })
            }
            Ok(None) => {
                debug!(
                    "Player {} not found in database making a new record",
                    username
                );

                let player = Player::new(username.clone(), edition);

                debug!("Inserting player {} into mongodb", username);

                player.insert(&players).await.map_err(|e| {
                    error!("Failed to insert player: {}", e);
                    Status::internal(format!("Failed to insert player: {}", e))
                })?;

                debug!("Inserting player {} into cache", username);

                self.cache.insert_player(player).await.map_err(|err| {
                    error!("Failed to insert player into cache: {}", err);
                    Status::internal(format!("Failed to insert player into cache: {}", err))
                })?;

                debug!(
                    "Successfully inserted player {} in cache and mongodb",
                    username
                );

                info!("Player {} joined the server", username);

                Response::new(PlayerJoinResponse { success: true })
            }
            Err(err) => {
                error!(
                    "Failed to find find player in database: {}",
                    err.to_string()
                );
                Response::new(PlayerJoinResponse { success: false })
            }
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::BridgeService;
    use crate::logger::Logger;
    use crate::{bridge::bridge_server::Bridge, models::player::Edition};
    use mongodb::bson::doc;

    #[tokio::test]
    async fn test_join_player() {
        Logger::init(true).await;

        let service = BridgeService::new().await;

        let username = "vaibhav_57887".to_string();
        let edition = 1;

        let req = tonic::Request::new(crate::bridge::PlayerJoinRequest {
            username: username.clone(),
            edition,
        });

        let resp = service.player_join(req).await.unwrap().into_inner();

        assert!(resp.success);

        let player = service.cache.get_player(&username).await.unwrap().unwrap();

        service
            .databases
            .players
            .delete_one(doc! {"username": &username})
            .await
            .unwrap();

        assert_eq!(player.edition, Edition::Java);
    }
}
