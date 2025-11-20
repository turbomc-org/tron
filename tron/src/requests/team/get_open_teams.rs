use crate::BridgeService;
use tonic::{Request, Response, Status};
use tron_protos::{GetOpenTeamsRequest, GetOpenTeamsResponse};

impl BridgeService {
    pub async fn handle_get_open_teams(
        &self,
        _request: Request<GetOpenTeamsRequest>,
    ) -> Result<Response<GetOpenTeamsResponse>, Status> {
        let open_teams: Vec<String> = self
            .state()
            .indexes
            .open_team
            .iter()
            .filter_map(|open_team_id| {
                self.state()
                    .teams
                    .get(&open_team_id)
                    .map(|t| t.name.clone())
            })
            .collect();

        Ok(Response::new(GetOpenTeamsResponse { teams: open_teams }))
    }
}
