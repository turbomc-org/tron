use crate::BridgeService;
use crate::bridge::{PlayerLeaveRequest, PlayerLeaveResponse};
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_player_leave(
        &self,
        request: Request<PlayerLeaveRequest>,
    ) -> Result<Response<PlayerLeaveResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let players_cache = &self.cache.active_players.clone();

        if players_cache.contains_key(&username) {
            players_cache.remove(&username);
            return Ok(Response::new(PlayerLeaveResponse { success: false }));
        } else {
            error!("Player {} is not in cache but still was the game", username);
            return Ok(Response::new(PlayerLeaveResponse { success: false }));
        }
    }
}
