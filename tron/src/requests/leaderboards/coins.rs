use crate::BridgeService;
use crate::bridge::{CoinsLeaderboardRequest, CoinsLeaderboardResponse};
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::{debug, error};

impl BridgeService {
    #[tracing::instrument]
    pub async fn handle_coins_leaderboard(
        &self,
        _request: Request<CoinsLeaderboardRequest>,
    ) -> Result<Response<CoinsLeaderboardResponse>, Status> {
        debug!("Coins leaderboard request received");

        let filtered_players = self
            .collections
            .players
            .get_leaderboard("coins", Some(10))
            .await
            .map_err(|err| {
                error!("Failed to fetch the leaderboard: {}", err);
                Status::internal("Failed to fetch the leaderboard")
            })?;

        let players: HashMap<String, u64> = filtered_players
            .into_iter()
            .map(|player| (player.username, player.coins))
            .collect();

        debug!("Coins leaderboard request completed");

        Ok(Response::new(CoinsLeaderboardResponse {
            leaderboard: players,
        }))
    }
}
