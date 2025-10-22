use crate::BridgeService;
use crate::bridge::{TeamsLeaderboardRequest, TeamsLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_teams_leaderboard(
        &self,
        _request: Request<TeamsLeaderboardRequest>,
    ) -> Result<Response<TeamsLeaderboardResponse>, Status> {
        unimplemented!()
    }
}
