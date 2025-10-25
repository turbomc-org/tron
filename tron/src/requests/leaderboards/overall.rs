use crate::BridgeService;
use crate::bridge::{OverallLeaderboardRequest, OverallLeaderboardResponse};
use crate::models::leaderboards::Leaderboards;
use crate::models::player::Player;
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_overall_leaderboard(
        &self,
        _request: Request<OverallLeaderboardRequest>,
    ) -> Result<Response<OverallLeaderboardResponse>, Status> {
        let filtered_players = Leaderboards::get_players_overall(&self.databases.players, Some(10))
            .await
            .map_err(|err| {
                error!("Failed to fetch the leaderboard: {}", err);
                Status::internal("Failed to fetch the leaderboard")
            })?;

        let players: HashMap<String, u64> = filtered_players
            .into_iter()
            .map(|player| {
                let kd_ratio = if player.deaths == 0 {
                    player.kills as f64
                } else {
                    player.kills as f64 / player.deaths as f64
                };

                let score = player.kills as f64
                    + (kd_ratio * 100.0)
                    + player.coins as f64
                    + (Player::get_rank_value(&player.rank) as f64 * 1000.0)
                    + player.vault_count as f64;

                (player.username, score.round() as u64)
            })
            .collect();

        Ok(Response::new(OverallLeaderboardResponse {
            leaderboard: players,
        }))
    }
}
