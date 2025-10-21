use crate::BridgeService;
use crate::bridge::{PlayerJoinRequest, PlayerJoinResponse};
use crate::models::player::Edition;
use crate::models::player::Player;
use tonic::{Request, Response, Status};
use tracing::error;

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

        if players_cache.contains_key(&username) {
            return Ok(Response::new(PlayerJoinResponse { success: false }));
        }

        let response = match Player::find_by_username(username.clone(), &players).await {
            Ok(Some(player)) => {
                &self.cache.insert_player(player).await;
                Response::new(PlayerJoinResponse { success: true })
            }
            Ok(None) => {
                let player = Player::new(username, edition);
                player
                    .insert(&players)
                    .await
                    .map_err(|e| Status::internal(format!("Failed to insert player: {}", e)));
                self.cache.insert_player(player);

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
