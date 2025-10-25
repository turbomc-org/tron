use crate::BridgeService;
use crate::bridge::{TeamsLeaderboardRequest, TeamsLeaderboardResponse};
use crate::models::leaderboards::Leaderboards;
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_teams_leaderboard(
        &self,
        _request: Request<TeamsLeaderboardRequest>,
    ) -> Result<Response<TeamsLeaderboardResponse>, Status> {
        let teams = Leaderboards::get_teams_overall(
            &self.databases.teams,
            &self.databases.players,
            Some(10),
        )
        .await
        .map_err(|err| {
            error!("Failed to fetch team overall leaderboard: {}", err);
            Status::internal("Failed to fetch team overall leaderboard")
        })?;

        let leaderboard: HashMap<String, u64> = teams
            .into_iter()
            .map(|(team, score)| (team.name, score.round() as u64))
            .collect();

        Ok(Response::new(TeamsLeaderboardResponse { leaderboard }))
    }
}
