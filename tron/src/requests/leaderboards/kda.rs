use crate::BridgeService;
use crate::bridge::{KdaLeaderboardRequest, KdaLeaderboardResponse};
use crate::models::leaderboards::Leaderboards;
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_kda_leaderboard(
        &self,
        _request: Request<KdaLeaderboardRequest>,
    ) -> Result<Response<KdaLeaderboardResponse>, Status> {
        let filtered_players = Leaderboards::get_players_kd(&self.databases.players, Some(10))
            .await
            .map_err(|err| {
                error!("Failed to fetch the leaderboard: {}", err);
                Status::internal("Failed to fetch the leaderboard")
            })?;

        let players: HashMap<String, f32> = filtered_players
            .into_iter()
            .map(|player| (player.username, player.kills as f32 / player.deaths as f32))
            .collect();

        Ok(Response::new(KdaLeaderboardResponse {
            leaderboard: players,
        }))
    }
}
