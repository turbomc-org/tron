use crate::BridgeService;
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{TeamsLeaderboardRequest, TeamsLeaderboardResponse};

impl BridgeService {
    pub async fn handle_teams_leaderboard(
        &self,
        _request: Request<TeamsLeaderboardRequest>,
    ) -> Result<Response<TeamsLeaderboardResponse>, Status> {
        info!("Team leaderboard request received");

        let mut leaderboard_with_names: HashMap<String, u64> = HashMap::new();
        let leaderboard = self.state().leaderboards.overall.get(10).await;

        for player in leaderboard {
            let username = self
                .state()
                .get_player_username(&player.0)
                .ok_or_else(|| Status::not_found("Player not found"))?;
            leaderboard_with_names.insert(username, player.1);
        }

        let players = leaderboard_with_names.into_iter().collect();

        info!("Team leaderboard request completed");

        Ok(Response::new(TeamsLeaderboardResponse {
            leaderboard: players,
        }))
    }
}
