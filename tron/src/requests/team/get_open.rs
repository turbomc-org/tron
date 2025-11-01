use crate::BridgeService;
use crate::bridge::{GetOpenTeamsRequest, GetOpenTeamsResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_open_teams(
        &self,
        _request: Request<GetOpenTeamsRequest>,
    ) -> Result<Response<GetOpenTeamsResponse>, Status> {
        let open_teams: Vec<String> = self
            .state
            .open_team_indexes
            .iter()
            .filter_map(|open_team_id| self.state.teams.get(&open_team_id).map(|t| t.name.clone()))
            .collect();

        Ok(Response::new(GetOpenTeamsResponse { teams: open_teams }))
    }
}
