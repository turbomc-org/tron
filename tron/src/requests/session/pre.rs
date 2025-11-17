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
        let current_edition = Edition::try_from(inner_request.edition).unwrap();

        if self.state().active_players.contains_key(&username) {
            return Err(Status::already_exists("Player is already logged in"));
        }

        let player = match self.collections().players.find_by_username(&username).await {
            Ok(Some(mut found_player)) => {
                if found_player.edition != current_edition {
                    info!(
                        "Player '{}' switched edition from {:?} to {:?}. Updating record.",
                        username, found_player.edition, current_edition
                    );
                    found_player.edition = current_edition;
                }
                found_player
            }
            Ok(None) => {
                let alias = name_generator::generate();
                let new_player = Player::new(username.clone(), Some(alias), current_edition);

                if let Err(e) = new_player
                    .insert(&self.collections().players, &self.state())
                    .await
                {
                    error!("Failed to insert player: {}", e);
                    return Err(Status::internal("Failed to insert player"));
                }
                new_player
            }
            Err(e) => {
                error!("Failed to find player with username: {}", e);
                return Err(Status::internal("Failed to register player."));
            }
        };

        let login_type: i32 = {
            if current_edition == Edition::Bedrock {
                0
            } else {
                1
            }
        };

        let response_username = if current_edition == Edition::Java {
            player
                .alias
                .clone()
                .unwrap_or_else(|| player.username.clone())
        } else {
            player.username.clone()
        };

        info!(
            "Player '{}' joined the game with login type: {}",
            player.username, login_type
        );

        if let Err(e) = self.state().insert_player(player.clone()).await {
            error!("Failed to insert player into active state: {}", e);
            return Err(Status::internal("Failed to activate player session"));
        }

        Ok(Response::new(PlayerPreLoginResponse {
            login_type,
            username: response_username,
        }))
    }
}
