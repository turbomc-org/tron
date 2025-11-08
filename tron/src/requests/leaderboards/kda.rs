use crate::BridgeService;
use crate::bridge::{KdaLeaderboardRequest, KdaLeaderboardResponse};
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_kda_leaderboard(
        &self,
        request: Request<KdaLeaderboardRequest>,
    ) -> Result<Response<KdaLeaderboardResponse>, Status> {
        info!("Kda leaderboard request received");

        let mut leaderboard_with_names: HashMap<String, f32> = HashMap::new();
        let leaderboard = self.state().leaderboards.kd.get(10).await;

        for player in leaderboard {
            let username = self
                .state()
                .get_player_username(&player.0)
                .ok_or_else(|| Status::not_found("Player not found"))?;
            leaderboard_with_names.insert(username, player.1 as f32);
        }

        let players = leaderboard_with_names.into_iter().collect();

        info!("Kda leaderboard request completed");

        Ok(Response::new(KdaLeaderboardResponse {
            leaderboard: players,
        }))
    }
}
