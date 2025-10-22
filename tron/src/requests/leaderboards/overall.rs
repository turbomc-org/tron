use crate::BridgeService;
use crate::bridge::{OverallLeaderboardRequest, OverallLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_overall_leaderboard(
        &self,
        _request: Request<OverallLeaderboardRequest>,
    ) -> Result<Response<OverallLeaderboardResponse>, Status> {
        unimplemented!()
    }
}
