use tonic::Status;

use crate::BridgeService;
use crate::models::player::Player;

impl BridgeService {
    pub async fn player(&self, username: &String) -> Result<Player, Status> {
        let player = self
            .state()
            .active_players
            .get(username)
            .map(|entry| entry.clone());
        match player {
            Some(player) => Ok(player),
            None => {
                return self
                    .status(
                        username,
                        Status::not_found(format!("Player {} not found", username)),
                    )
                    .await;
            }
        }
    }
}
