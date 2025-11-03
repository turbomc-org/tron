use crate::BridgeService;
use crate::bridge::{GetAllTeamsRequest, GetAllTeamsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_all_teams(
        &self,
        _request: Request<GetAllTeamsRequest>,
    ) -> Result<Response<GetAllTeamsResponse>, Status> {
        let teams: Vec<String> = self.state.teams.iter().map(|t| t.name.clone()).collect();
        Ok(Response::new(GetAllTeamsResponse { teams }))
    }
}
