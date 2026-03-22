use crate::BridgeService;
use crate::models::player::{Edition, Player};
use tonic::{Request, Response, Status};
use tracing::error;
use tron_protos::{PlayerLoginRequest, PlayerLoginResponse};

impl BridgeService {
    pub async fn handle_player_login(
        &self,
        request: Request<PlayerLoginRequest>,
    ) -> Result<Response<PlayerLoginResponse>, Status> {
        let request = request.into_inner();
        let username = request.username;
        let edition = Edition::try_from(request.edition).unwrap();

        let player = match self.collections().players.find_by_username(&username).await {
            Ok(Some(player)) => player,
            Ok(None) => {
                let player = Player::new(username, edition);
                if let Err(e) = player
                    .insert(&self.collections().players, &self.state())
                    .await
                {
                    error!("Failed to register the player: {}", e);
                    return Err(Status::internal("Failed to authenticate the player."));
                }

                player
            }
            Err(e) => {
                error!("Failed to register the player: {}", e);
                return Err(Status::internal("Failed to authenticate the player."));
            }
        };

        self.state()
            .active_players
            .insert(player.username.clone(), player.clone());

        self.state().messaging.join_global(player.id);

        Ok(Response::new(PlayerLoginResponse { success: true }))
    }
}
