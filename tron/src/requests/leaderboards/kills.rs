use crate::BridgeService;
use crate::bridge::{KillsLeaderboardRequest, KillsLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_kills_leaderboard(
        &self,
        _request: Request<KillsLeaderboardRequest>,
    ) -> Result<Response<KillsLeaderboardResponse>, Status> {
        unimplemented!()
    }
}
