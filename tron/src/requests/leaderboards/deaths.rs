use crate::BridgeService;
use crate::bridge::{DeathsLeaderboardRequest, DeathsLeaderboardResponse};
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    pub async fn handle_deaths_leaderboard(
        &self,
        _request: Request<DeathsLeaderboardRequest>,
    ) -> Result<Response<DeathsLeaderboardResponse>, Status> {
        info!("Deaths leaderboard request received");

        let mut leaderboard_with_names: HashMap<String, u64> = HashMap::new();
        let leaderboard = self.state().leaderboards.deaths.get(10).await;

        for player in leaderboard {
            let username = self
                .state()
                .get_player_username(&player.0)
                .ok_or_else(|| Status::not_found("Player not found"))?;
            leaderboard_with_names.insert(username, player.1);
        }

        let players = leaderboard_with_names.into_iter().collect();

        info!("Deaths leaderboard request completed");

        Ok(Response::new(DeathsLeaderboardResponse {
            leaderboard: players,
        }))
    }
}
