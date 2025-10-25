use crate::BridgeService;
use crate::bridge::{KillsLeaderboardRequest, KillsLeaderboardResponse};
use crate::models::leaderboards::Leaderboards;
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_kills_leaderboard(
        &self,
        _request: Request<KillsLeaderboardRequest>,
    ) -> Result<Response<KillsLeaderboardResponse>, Status> {
        let filtered_players =
            Leaderboards::get_players(&self.databases.players, "kills", Some(10))
                .await
                .map_err(|err| {
                    error!("Failed to fetch the leaderboard: {}", err);
                    Status::internal("Failed to fetch the leaderboard")
                })?;

        let players: HashMap<String, u64> = filtered_players
            .into_iter()
            .map(|player| (player.username, player.kills))
            .collect();

        Ok(Response::new(KillsLeaderboardResponse {
            leaderboard: players,
        }))
    }
}
