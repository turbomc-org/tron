use crate::BridgeService;
use crate::bridge::{GetScoreboardStatusRequest, GetScoreboardStatusResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_scoreboard_status(
        &self,
        request: Request<GetScoreboardStatusRequest>,
    ) -> Result<Response<GetScoreboardStatusResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.state().get_player_with_handling(&username).await?;

        Ok(Response::new(GetScoreboardStatusResponse {
            enabled: player.scoreboard_enabled,
        }))
    }
}
