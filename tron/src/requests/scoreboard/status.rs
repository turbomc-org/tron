use crate::BridgeService;
use tonic::{Request, Response, Status};
use tron_protos::{GetScoreboardStatusRequest, GetScoreboardStatusResponse};

impl BridgeService {
    pub async fn handle_get_scoreboard_status(
        &self,
        request: Request<GetScoreboardStatusRequest>,
    ) -> Result<Response<GetScoreboardStatusResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.player(&username).await?;

        Ok(Response::new(GetScoreboardStatusResponse {
            enabled: player.scoreboard_enabled,
        }))
    }
}
