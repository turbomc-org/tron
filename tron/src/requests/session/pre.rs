use crate::BridgeService;
use crate::bridge::{PlayerPreLoginRequest, PlayerPreLoginResponse};
use crate::models::player::{Edition, Player};
use crate::utils::name_generator;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_player_pre_login(
        &self,
        request: Request<PlayerPreLoginRequest>,
    ) -> Result<Response<PlayerPreLoginResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let edition = inner_request.edition;

        if self.state().active_players.contains_key(&username) {
            return Err(Status::already_exists("Player is already logged in"));
        }

        let player = match self.collections().players.find_by_username(&username).await {
            Ok(Some(player)) => player,
            Ok(None) => {
                let alias = name_generator::generate();
                let edition = Edition::try_from(edition).unwrap();
                let player = Player::new(alias, Some(username), edition);

                if let Err(e) = player
                    .insert(&self.collections().players, &self.state())
                    .await
                {
                    error!("Failed to insert player: {}", e);
                    return Err(Status::internal("Failed to insert player"));
                }

                player
            }
            Err(e) => {
                error!("Failed to find player with username: {}", e);
                return Err(Status::internal("Failed to register player."));
            }
        };

        let login_type: i32 = {
            if player.edition == Edition::Bedrock {
                0
            } else if player.original_name.is_some() {
                1
            } else if player.password.is_some() {
                2
            } else {
                return Err(Status::internal("Player is not registered"));
            }
        };

        info!("Player {} joined the game", player.username);

        if let Err(e) = self.state().insert_player(player.clone()).await {
            error!("Failed to insert player: {}", e);
            return Err(Status::internal("Failed to insert player"));
        }

        Ok(Response::new(PlayerPreLoginResponse {
            login_type,
            username: player.username,
        }))
    }
}
