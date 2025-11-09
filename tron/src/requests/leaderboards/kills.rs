use crate::BridgeService;
use crate::bridge::{KillsLeaderboardRequest, KillsLeaderboardResponse};
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_kills_leaderboard(
        &self,
        request: Request<KillsLeaderboardRequest>,
    ) -> Result<Response<KillsLeaderboardResponse>, Status> {
        info!("Kills leaderboard request received");

        let mut leaderboard_with_names: HashMap<String, u64> = HashMap::new();
        let leaderboard = self.state().leaderboards.kills.get(10).await;

        for player in leaderboard {
            let username = self
                .state()
                .get_player_username(&player.0)
                .ok_or_else(|| Status::not_found("Player not found"))?;
            leaderboard_with_names.insert(username, player.1);
        }

        let players = leaderboard_with_names.into_iter().collect();

        info!("Kills leaderboard request completed");

        Ok(Response::new(KillsLeaderboardResponse {
            leaderboard: players,
        }))
    }
}
