use crate::BridgeService;
use crate::bridge::{ListOverallLeaderboardRequest, ListOverallLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_overall_leaderboard(
        &self,
        request: Request<ListOverallLeaderboardRequest>,
    ) -> Result<Response<ListOverallLeaderboardResponse>, Status> {
        todo!("Implement list coins leaderboard")
    }
}
