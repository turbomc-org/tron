use crate::BridgeService;
use crate::bridge::{GetScoreboardRequest, GetScoreboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_scoreboard(
        &self,
        request: Request<GetScoreboardRequest>,
    ) -> Result<Response<GetScoreboardResponse>, Status> {
        todo!("Implement get scoreboard")
    }
}
