use crate::BridgeService;
use crate::bridge::{ListDeathsLeaderboardRequest, ListDeathsLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_deaths_leaderboard(
        &self,
        request: Request<ListDeathsLeaderboardRequest>,
    ) -> Result<Response<ListDeathsLeaderboardResponse>, Status> {
        todo!("Implement list coins leaderboard")
    }
}
