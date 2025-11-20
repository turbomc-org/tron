use crate::BridgeService;
use crate::models::player::Role;
use tonic::{Request, Response, Status};
use tron_protos::{GetAllServersRequest, GetAllServersResponse};

impl BridgeService {
    pub async fn handle_get_all_servers(
        &self,
        request: Request<GetAllServersRequest>,
    ) -> Result<Response<GetAllServersResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.player(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use get all server command."),
                )
                .await;
        }

        let names: Vec<String> = self
            .state()
            .servers
            .documents
            .iter()
            .map(|r| r.name.clone())
            .collect();

        Ok(Response::new(GetAllServersResponse { names }))
    }
}
