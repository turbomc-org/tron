use crate::BridgeService;
use crate::bridge::{ListKillsLeaderboardRequest, ListKillsLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_kills_leaderboard(
        &self,
        request: Request<ListKillsLeaderboardRequest>,
    ) -> Result<Response<ListKillsLeaderboardResponse>, Status> {
        todo!("Implement list coins leaderboard")
    }
}
