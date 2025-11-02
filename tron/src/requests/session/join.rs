use crate::BridgeService;
use crate::bridge::{PlayerJoinRequest, PlayerJoinResponse, ServerSendMessageResponse};
use crate::models::player::Edition;
use crate::models::player::Player;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_player_join(
        &self,
        request: Request<PlayerJoinRequest>,
    ) -> Result<Response<PlayerJoinResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        #[allow(deprecated)]
        let edition: Edition =
            crate::bridge::player_join_request::Edition::from_i32(inner_request.edition)
                .unwrap_or(crate::bridge::player_join_request::Edition::Java)
                .into();

        debug!("Join request for player {} received", username);

        #[cfg(not(debug_assertions))]
        if self.cache.active_players.contains_key(&username) {
            error!("Player {} already exists in cache", username);
            return Ok(Response::new(PlayerJoinResponse { success: false }));
        }

        debug!("Fetching player {} record from mongodb", username);

        let response = match self.collections.players.find_by_username(&username).await {
            Ok(Some(player)) => {
                debug!("Inserting player {} into cache", username);

                self.state
                    .insert_player(player.clone())
                    .await
                    .map_err(|err| {
                        error!("Failed to insert player into cache: {}", err);
                        Status::internal(format!("Failed to insert player into cache: {}", err))
                    })?;

                debug!("Successfully inserted player {} into cache", username);

                self.broadcast_message(ServerSendMessageResponse {
                    username: player.username,
                    timestamp: 0,
                    message: format!("Welcome, <rainbow>{}</rainbow>!", username),
                })
                .await;

                info!("Player {} joined the server", username);
                Response::new(PlayerJoinResponse { success: true })
            }
            Ok(None) => {
                debug!(
                    "Player {} not found in database making a new record",
                    username
                );

                let player = Player::new(username.clone(), edition);

                debug!("Inserting player {} into cache and database", username);

                player
                    .insert(&self.collections.players, &self.state)
                    .await
                    .map_err(|e| {
                        error!("Failed to insert player: {}", e);
                        Status::internal(format!("Failed to insert player: {}", e))
                    })?;

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
