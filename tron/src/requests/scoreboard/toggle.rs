use crate::BridgeService;
use crate::bridge::{ToggleScoreboardRequest, ToggleScoreboardResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_toggle_scoreboard(
        &self,
        request: Request<ToggleScoreboardRequest>,
    ) -> Result<Response<ToggleScoreboardResponse>, Status> {
        todo!("Implement toggle scoreboard")
    }
}
