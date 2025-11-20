use crate::BridgeService;
use tonic::{Request, Response, Status};
use tron_protos::{GetTeamRequestsRequest, GetTeamRequestsResponse};

impl BridgeService {
    pub async fn handle_get_team_requests(
        &self,
        request: Request<GetTeamRequestsRequest>,
    ) -> Result<Response<GetTeamRequestsResponse>, Status> {
        let request = request.into_inner();
        let username = request.username;

        let player = self.player(&username).await?;
        let teams: Vec<String> = player
            .incoming_team_requests
            .iter()
            .map(|v| {
                self.state()
                    .get_team_name(v.0.clone())
                    .unwrap_or("Unknown".to_string())
            })
            .collect();

        Ok(Response::new(GetTeamRequestsResponse { teams }))
    }
}
