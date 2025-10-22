use crate::BridgeService;
use crate::bridge::{KdaLeaderboardRequest, KdaLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_kda_leaderboard(
        &self,
        _request: Request<KdaLeaderboardRequest>,
    ) -> Result<Response<KdaLeaderboardResponse>, Status> {
        unimplemented!()
    }
}
