use crate::BridgeService;
use crate::bridge::{GetServerRequest, GetServerResponse, Server as CompiledServer};
use crate::models::player::Role;
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_server(
        &self,
        request: Request<GetServerRequest>,
    ) -> Result<Response<GetServerResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let server_name = inner_request.name;

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use get server command."),
                )
                .await;
        }

        if !self.state().servers.names.contains_key(&server_name) {
            return self
                .status(
                    &username,
                    Status::not_found(format!("Server with name {} not found.", server_name)),
                )
                .await;
        }

        let server_id = match self.state().servers.names.get(&server_name) {
            Some(ref_multi) => *ref_multi.value(),
            None => {
                return self
                    .status(
                        &username,
                        Status::not_found(format!("Server with name {} not found.", server_name)),
                    )
                    .await;
            }
        };

        let server = match self.state().servers.documents.get(&server_id) {
            Some(ref_multi) => ref_multi.value().clone(),
            None => {
                return self
                    .status(
                        &username,
                        Status::not_found(format!("Server with id {} not found.", server_id)),
                    )
                    .await;
            }
        };

        Ok(Response::new(GetServerResponse {
            server: Some(CompiledServer {
                id: server.id,
                name: server.name,
                description: server.description,
                address: server.address,
            }),
        }))
    }
}
