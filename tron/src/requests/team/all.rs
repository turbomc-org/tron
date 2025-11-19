use crate::BridgeService;
use tonic::{Request, Response, Status};
use tron_protos::{GetAllTeamsRequest, GetAllTeamsResponse};

impl BridgeService {
    pub async fn handle_get_all_teams(
        &self,
        _request: Request<GetAllTeamsRequest>,
    ) -> Result<Response<GetAllTeamsResponse>, Status> {
        let teams: Vec<String> = self.state().teams.iter().map(|t| t.name.clone()).collect();
        Ok(Response::new(GetAllTeamsResponse { teams }))
    }
}
