use crate::BridgeService;
use crate::bridge::{GetLeaderboardRequest, GetLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_leaderboard(
        &self,
        request: Request<GetLeaderboardRequest>,
    ) -> Result<Response<GetLeaderboardResponse>, Status> {
        unimplemented!()
    }
}
