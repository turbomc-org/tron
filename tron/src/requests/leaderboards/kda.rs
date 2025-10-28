use crate::BridgeService;
use crate::bridge::{KdaLeaderboardRequest, KdaLeaderboardResponse};
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_kda_leaderboard(
        &self,
        _request: Request<KdaLeaderboardRequest>,
    ) -> Result<Response<KdaLeaderboardResponse>, Status> {
        let filtered_players = self
            .collections
            .players
            .get_leaderboard_kda(Some(10))
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
