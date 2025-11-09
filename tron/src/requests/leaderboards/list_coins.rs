use crate::BridgeService;
use crate::bridge::{ListCoinsLeaderboardRequest, ListCoinsLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_coins_leaderboard(
        &self,
        request: Request<ListCoinsLeaderboardRequest>,
    ) -> Result<Response<ListCoinsLeaderboardResponse>, Status> {
        todo!("Implement list coins leaderboard")
    }
}
