use crate::BridgeService;
use crate::bridge::{ListTeamsLeaderboardRequest, ListTeamsLeaderboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_teams_leaderboard(
        &self,
        request: Request<ListTeamsLeaderboardRequest>,
    ) -> Result<Response<ListTeamsLeaderboardResponse>, Status> {
        todo!("Implement list coins leaderboard")
    }
}
