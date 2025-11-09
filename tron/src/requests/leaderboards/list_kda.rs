use crate::BridgeService;
use crate::bridge::{ListKdaLeaderboardRequest, ListKdaLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_kda_leaderboard(
        &self,
        request: Request<ListKdaLeaderboardRequest>,
    ) -> Result<Response<ListKdaLeaderboardResponse>, Status> {
        todo!("Implement list coins leaderboard")
    }
}
