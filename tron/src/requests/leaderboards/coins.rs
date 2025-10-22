use crate::BridgeService;
use crate::bridge::{CoinsLeaderboardRequest, CoinsLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_coins_leaderboard(
        &self,
        _request: Request<CoinsLeaderboardRequest>,
    ) -> Result<Response<CoinsLeaderboardResponse>, Status> {
        unimplemented!()
    }
}
