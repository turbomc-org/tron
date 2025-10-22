use crate::BridgeService;
use crate::bridge::{DeathsLeaderboardRequest, DeathsLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_deaths_leaderboard(
        &self,
        _request: Request<DeathsLeaderboardRequest>,
    ) -> Result<Response<DeathsLeaderboardResponse>, Status> {
        unimplemented!()
    }
}
