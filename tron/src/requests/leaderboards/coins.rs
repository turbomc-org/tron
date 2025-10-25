use crate::BridgeService;
use crate::bridge::{CoinsLeaderboardRequest, CoinsLeaderboardResponse};
use crate::models::leaderboards::Leaderboards;
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_coins_leaderboard(
        &self,
        _request: Request<CoinsLeaderboardRequest>,
    ) -> Result<Response<CoinsLeaderboardResponse>, Status> {
        let filtered_players =
            Leaderboards::get_players(&self.databases.players, "coins", Some(10))
                .await
                .map_err(|err| {
                    error!("Failed to fetch the leaderboard: {}", err);
                    Status::internal("Failed to fetch the leaderboard")
                })?;

        let players: HashMap<String, u64> = filtered_players
            .into_iter()
            .map(|player| (player.username, player.coins))
            .collect();

        Ok(Response::new(CoinsLeaderboardResponse {
            leaderboard: players,
        }))
    }
}
