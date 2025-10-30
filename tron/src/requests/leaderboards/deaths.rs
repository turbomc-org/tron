use crate::BridgeService;
use crate::bridge::{DeathsLeaderboardRequest, DeathsLeaderboardResponse};
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::{debug, error};

impl BridgeService {
    #[tracing::instrument]
    pub async fn handle_deaths_leaderboard(
        &self,
        _request: Request<DeathsLeaderboardRequest>,
    ) -> Result<Response<DeathsLeaderboardResponse>, Status> {
        debug!("Deaths leaderboard request received");

        let filtered_players = self
            .collections
            .players
            .get_leaderboard("deaths", Some(10))
            .await
            .map_err(|err| {
                error!("Failed to fetch the leaderboard: {}", err);
                Status::internal("Failed to fetch the leaderboard")
            })?;

        let players: HashMap<String, u64> = filtered_players
            .into_iter()
            .map(|player| (player.username, player.deaths))
            .collect();

        debug!("Deaths leaderboard request completed");

        Ok(Response::new(DeathsLeaderboardResponse {
            leaderboard: players,
        }))
    }
}
